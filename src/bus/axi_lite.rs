use super::MemMap;
use crate::{
    module::Module,
    stmt::{Case, Stmt},
    util::range,
};

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct AXILite {
    _name: String,
    clk: String,
    rst: String,

    // Write Addr
    awaddr: String,
    awvalid: String,
    awready: String,

    // Write Data
    wdata: String,
    wstrb: String,
    wvalid: String,
    wready: String,

    // Write Response
    bresp: String,
    bvalid: String,
    bready: String,

    // Read Addr
    araddr: String,
    arvalid: String,
    arready: String,

    // Read Data
    rdata: String,
    rresp: String,
    rvalid: String,
    rready: String,
}

impl AXILite {
    fn new(name: Option<&str>, clk: impl ToString, rst: impl ToString) -> Self {
        let name: String = name
            .map(|n| format!("{}_", n.to_string()))
            .unwrap_or(format!(""));
        Self {
            _name: name.clone(),
            clk: clk.to_string(),
            rst: rst.to_string(),
            awaddr: format!("{name}awaddr"),
            awvalid: format!("{name}awvalid"),
            awready: format!("{name}awready"),
            wdata: format!("{name}wdata"),
            wstrb: format!("{name}wstrb"),
            wvalid: format!("{name}wvalid"),
            wready: format!("{name}wready"),
            bresp: format!("{name}bresp"),
            bvalid: format!("{name}bvalid"),
            bready: format!("{name}bready"),
            araddr: format!("{name}araddr"),
            arvalid: format!("{name}arvalid"),
            arready: format!("{name}arready"),
            rdata: format!("{name}rdata"),
            rresp: format!("{name}rresp"),
            rvalid: format!("{name}rvalid"),
            rready: format!("{name}rready"),
        }
    }
}

impl Module {
    pub fn axi_lite_slave(
        mut self,
        name: Option<&str>,
        clk: impl ToString,
        rst: impl ToString,
        mem: MemMap,
    ) -> Self {
        let bus = AXILite::new(name, clk, rst);

        // Regs
        self = self.define_regs(&mem);

        // IO Port
        self = self
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
        self = self.sync_ff(
            init,
            Stmt::begin()
                .r#if(
                    &format!("{} && {}", bus.wvalid, bus.awvalid),
                    Stmt::begin().case(case).end(),
                )
                .end(),
        );

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
        self = self.sync_ff(
            Stmt::assign(&bus.rdata, "0"),
            Stmt::begin()
                .r#if(&bus.arvalid, Stmt::begin().case(case).end())
                .end(),
        );

        // AXI Lite Protocol
        self = self.sync_ff(
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
        );

        self
    }
}
