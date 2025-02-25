use super::MemMap;
use crate::{
    module::Module,
    stmt::{Case, Stmt},
    util::range,
};

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Pico {
    _name: String,
    clk: String,
    rst: String,

    // Bus wire names
    ready: String,
    valid: String,
    addr: String,
    wstrb: String,
    wdata: String,
    rdata: String,
}

impl Pico {
    fn new(name: impl ToString, clk: impl ToString, rst: impl ToString) -> Self {
        let name = name.to_string();
        Self {
            _name: name.clone(),
            clk: clk.to_string(),
            rst: rst.to_string(),
            ready: format!("{name}_ready"),
            valid: format!("{name}_valid"),
            addr: format!("{name}_addr"),
            wstrb: format!("{name}_wstrb"),
            wdata: format!("{name}_wdata"),
            rdata: format!("{name}_rdata"),
        }
    }
}

impl Module {
    pub fn pico_slave(
        mut self,
        name: impl ToString,
        clk: impl ToString,
        rst: impl ToString,
        mem: MemMap,
    ) -> Self {
        assert!(mem.data_bit == 32, "Data bit width must be 32");
        assert!(mem.addr_bit <= 32, "Addr bit width must be <= 32");

        let bus = Pico::new(name, clk, rst);

        // Regs
        self = self.define_regs(&mem);

        // IO Port
        self = self
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
        self = self.sync_ff(init, Stmt::begin().case(case).end());

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
        self = self.always_comb(Stmt::begin().case(case).end());

        self
    }

    pub fn pico_master(
        mut self,
        name: impl ToString,
        clk: impl ToString,
        rst: impl ToString,
        mem: MemMap,
    ) -> Self {
        self
    }
}
