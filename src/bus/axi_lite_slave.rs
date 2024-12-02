use super::{Entry, MMap};
use crate::{
    module::Module,
    stmt::{Case, Stmt},
    util::{clog2, range},
};

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AXILiteSlave {
    pub name: String,
    mmap: MMap,

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

impl AXILiteSlave {
    pub fn new(name: impl ToString, mmap: MMap) -> Self {
        assert!(
            mmap.data_width == 32 || mmap.data_width == 64,
            "AXI Lite supprots 32 or 64 bit data bus"
        );
        let name: String = name.to_string();
        Self {
            name: name.clone(),
            mmap,
            awaddr: format!("{name}_awaddr"),
            awvalid: format!("{name}_awvalid"),
            awready: format!("{name}_awready"),
            wdata: format!("{name}_wdata"),
            wstrb: format!("{name}_wstrb"),
            wvalid: format!("{name}_wvalid"),
            wready: format!("{name}_wready"),
            bresp: format!("{name}_bresp"),
            bvalid: format!("{name}_bvalid"),
            bready: format!("{name}_bready"),
            araddr: format!("{name}_araddr"),
            arvalid: format!("{name}_arvalid"),
            arready: format!("{name}_arready"),
            rdata: format!("{name}_rdata"),
            rresp: format!("{name}_rresp"),
            rvalid: format!("{name}_rvalid"),
            rready: format!("{name}_rready"),
        }
    }
}

impl Module {
    pub fn axi_lite_slave(
        mut self,
        clk: impl ToString + Clone,
        rst: impl ToString + Clone,
        bus: AXILiteSlave,
    ) -> Self {
        // Allocate Registors
        let (aloc, size) = {
            let mut addr = 0;
            let mut aloc = vec![];
            for entry in &bus.mmap.list {
                for idx in 0..entry.len() {
                    aloc.push(entry.allocate(addr, idx));
                    addr += 1;
                }
            }
            (aloc, addr)
        };
        let addr_width = clog2(size).unwrap_or(1);

        // IO Port
        self = self
            .input(&bus.awaddr, addr_width)
            .input(&bus.awvalid, 1)
            .output(&bus.awready, 1)
            .input(&bus.wdata, bus.mmap.data_width)
            .input(&bus.wstrb, bus.mmap.data_width / 8)
            .input(&bus.wvalid, 1)
            .output(&bus.wready, 1)
            .output(&bus.bresp, 2)
            .output(&bus.bvalid, 1)
            .input(&bus.bready, 1)
            .input(&bus.araddr, addr_width)
            .input(&bus.arvalid, 1)
            .output(&bus.arready, 1)
            .output(&bus.rdata, bus.mmap.data_width)
            .output(&bus.rresp, 2)
            .output(&bus.rvalid, 1)
            .input(&bus.rready, 1);

        // Regs
        for entry in &bus.mmap.list {
            self = match entry {
                Entry::ReadWrite { name, bit, len } => self.logic(name, *bit, *len),
                Entry::ReadOnly { name, bit, len } => self.logic(name, *bit, *len),
                Entry::Trigger { name } => {
                    self.logic(&format!("{name}_trig"), 1, 1)
                        .logic(&format!("{name}_resp"), 1, 1)
                }
            };
        }

        // Write Logic
        let init = {
            let mut stmt = Stmt::begin();
            for entry in &aloc {
                if let Some(name) = &entry.write {
                    stmt = stmt.assign(&name, "0");
                }
            }
            stmt.end()
        };
        let case = {
            let mut cases = Case::new(&bus.awaddr);
            for entry in &aloc {
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
            clk.clone(),
            rst.clone(),
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
            for entry in &aloc {
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
            clk.clone(),
            rst.clone(),
            Stmt::assign(&bus.rdata, "0"),
            Stmt::begin()
                .r#if(&bus.arvalid, Stmt::begin().case(case).end())
                .end(),
        );

        // AXI Lite Protocol
        self = self.sync_ff(
            clk,
            rst,
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
