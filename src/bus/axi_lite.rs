use super::MemMap;
use crate::{
    ext::DFF,
    module::{Extension, Module},
    stmt::{Case, Stmt},
    util::range,
};

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AXILiteSlave {
    name: Option<String>,
    clk: String,
    rst: String,
    mem: MemMap,
}

impl AXILiteSlave {
    pub fn new(name: Option<&str>, clk: impl ToString, rst: impl ToString, mem: MemMap) -> Self {
        Self {
            name: name.map(|s| s.to_string()),
            clk: clk.to_string(),
            rst: rst.to_string(),
            mem,
        }
    }

    fn signal_names(&self) -> SignalNames {
        let prefix = self
            .name
            .as_ref()
            .map(|n| format!("{}_", n))
            .unwrap_or_default();
        SignalNames {
            awaddr: format!("{prefix}awaddr"),
            awvalid: format!("{prefix}awvalid"),
            awready: format!("{prefix}awready"),
            wdata: format!("{prefix}wdata"),
            wstrb: format!("{prefix}wstrb"),
            wvalid: format!("{prefix}wvalid"),
            wready: format!("{prefix}wready"),
            bresp: format!("{prefix}bresp"),
            bvalid: format!("{prefix}bvalid"),
            bready: format!("{prefix}bready"),
            araddr: format!("{prefix}araddr"),
            arvalid: format!("{prefix}arvalid"),
            arready: format!("{prefix}arready"),
            rdata: format!("{prefix}rdata"),
            rresp: format!("{prefix}rresp"),
            rvalid: format!("{prefix}rvalid"),
            rready: format!("{prefix}rready"),
        }
    }
}

#[derive(Debug, Clone)]
struct SignalNames {
    awaddr: String,
    awvalid: String,
    awready: String,
    wdata: String,
    wstrb: String,
    wvalid: String,
    wready: String,
    bresp: String,
    bvalid: String,
    bready: String,
    araddr: String,
    arvalid: String,
    arready: String,
    rdata: String,
    rresp: String,
    rvalid: String,
    rready: String,
}

impl Extension for AXILiteSlave {
    fn add(self, mut module: Module) -> Module {
        let bus = self.signal_names();
        let mem = &self.mem;

        // Regs
        module = module.define_regs(mem);

        // IO Port
        module = module
            .input(&bus.awaddr, mem.addr_bit)
            .input(&bus.awvalid, 1)
            .output(&bus.awready, 1)
            .input(&bus.wdata, mem.data_bit)
            .input(&bus.wstrb, mem.data_bit / 8)
            .input(&bus.wvalid, 1)
            .output(&bus.wready, 1)
            .output(&bus.bresp, 2)
            .output(&bus.bvalid, 1)
            .input(&bus.bready, 1)
            .input(&bus.araddr, mem.addr_bit)
            .input(&bus.arvalid, 1)
            .output(&bus.arready, 1)
            .output(&bus.rdata, mem.data_bit)
            .output(&bus.rresp, 2)
            .output(&bus.rvalid, 1)
            .input(&bus.rready, 1);

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
            let mut cases = Case::new(&bus.awaddr);
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
        module = module.add(DFF::sync(
            init,
            Stmt::begin()
                .r#if(
                    &format!("{} && {}", bus.wvalid, bus.awvalid),
                    Stmt::begin().case(case).end(),
                )
                .end(),
        ));

        // Read Logic
        let case = {
            let mut cases = Case::new(&bus.araddr);
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
        module = module.add(DFF::sync(
            Stmt::assign(&bus.rdata, "0"),
            Stmt::begin()
                .r#if(&bus.arvalid, Stmt::begin().case(case).end())
                .end(),
        ));

        // AXI Lite Protocol
        module = module.add(DFF::sync(
            Stmt::begin()
                .assign(&bus.awready, "0")
                .assign(&bus.wready, "0")
                .assign(&bus.bvalid, "0")
                .assign(&bus.arready, "0")
                .assign(&bus.rvalid, "0")
                .assign(&bus.bresp, "0")
                .assign(&bus.rresp, "0")
                .end(),
            Stmt::begin()
                .assign(
                    &bus.awready,
                    &format!("{} && !{}", bus.awvalid, bus.awready),
                )
                .assign(&bus.wready, &format!("{} && !{}", bus.wvalid, bus.wready))
                .assign(
                    &bus.bvalid,
                    &format!("{} && {} && !{}", bus.awready, bus.wready, bus.bvalid),
                )
                .assign(
                    &bus.arready,
                    &format!("{} && !{}", bus.arvalid, bus.arready),
                )
                .assign(&bus.rvalid, &format!("{} && !{}", bus.arvalid, bus.arready))
                .r#if(
                    &format!("{} && {}", bus.bvalid, bus.bready),
                    Stmt::assign(&bus.bvalid, "0"),
                )
                .r#if(
                    &format!("{} && {}", bus.rvalid, bus.rready),
                    Stmt::assign(&bus.rvalid, "0"),
                )
                .end(),
        ));

        module
    }
}
