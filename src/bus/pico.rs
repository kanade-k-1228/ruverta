use super::MemMap;
use crate::{
    ext::DFF,
    module::{Extension, Module},
    stmt::{Case, Stmt},
    util::range,
};

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PicoSlave {
    name: String,
    clk: String,
    rst: String,
    mem: MemMap,
}

impl PicoSlave {
    pub fn new(name: impl ToString, clk: impl ToString, rst: impl ToString, mem: MemMap) -> Self {
        assert!(mem.data_bit == 32, "Data bit width must be 32");
        assert!(mem.addr_bit <= 32, "Addr bit width must be <= 32");

        Self {
            name: name.to_string(),
            clk: clk.to_string(),
            rst: rst.to_string(),
            mem,
        }
    }

    fn signal_names(&self) -> SignalNames {
        let name = &self.name;
        SignalNames {
            ready: format!("{name}_ready"),
            valid: format!("{name}_valid"),
            addr: format!("{name}_addr"),
            wstrb: format!("{name}_wstrb"),
            wdata: format!("{name}_wdata"),
            rdata: format!("{name}_rdata"),
        }
    }
}

#[derive(Debug, Clone)]
struct SignalNames {
    ready: String,
    valid: String,
    addr: String,
    wstrb: String,
    wdata: String,
    rdata: String,
}

impl Extension for PicoSlave {
    fn add(self, mut module: Module) -> Module {
        let bus = self.signal_names();
        let mem = &self.mem;

        // Regs
        module = module.define_regs(mem);

        // IO Port
        module = module
            .input(&bus.valid, 1)
            .input(&bus.ready, 1)
            .input(&bus.wstrb, mem.data_bit / 8)
            .input(&bus.addr, mem.addr_bit)
            .input(&bus.wdata, mem.data_bit)
            .output(&bus.rdata, mem.data_bit);

        // Write Logic
        let init = {
            let mut stmt = Stmt::begin();
            for entry in &mem.map {
                if let Some(name) = &entry.write {
                    stmt = stmt.assign(&name, "0");
                }
            }
            stmt.end()
        };
        let case = {
            let mut cases = Case::new(&bus.addr);
            for entry in &mem.map {
                if let Some(name) = &entry.write {
                    cases = cases.case(
                        &format!("{}", entry.addr),
                        Stmt::assign(name, &format!("{}{}", bus.wdata, range(entry.bit, 0))),
                    );
                }
            }
            cases.default(Stmt::empty())
        };
        module = module.add(DFF::sync(init, Stmt::begin().case(case).end()));

        // Read Logic
        let case = {
            let mut cases = Case::new(&bus.addr);
            for entry in &mem.map {
                if let Some(name) = &entry.read {
                    cases = cases.case(
                        &format!("{}", entry.addr),
                        Stmt::assign(&format!("{}{}", bus.rdata, range(entry.bit, 0)), name),
                    );
                }
            }
            cases.default(Stmt::assign(&bus.rdata, "0"))
        };
        module = module.always_comb(Stmt::begin().case(case).end());

        module
    }
}
