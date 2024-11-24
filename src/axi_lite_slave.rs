use crate::{
    module::Module,
    stmt::{Case, Stmt},
    util::{clog2, range, sel},
};

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AXILiteSlave {
    pub name: String,
    bit: usize,
    list: Vec<Entry>,

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
    pub fn new(name: impl ToString, bit: usize) -> Self {
        assert!(bit == 32 || bit == 64);
        let name: String = name.to_string();
        Self {
            name: name.clone(),
            bit,
            list: vec![],
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

    pub fn read_write(mut self, name: impl ToString, bit: usize, len: usize) -> Self {
        assert!(bit <= self.bit);
        self.list.push(Entry::read_write(name, bit, len));
        self
    }

    pub fn read_only(mut self, name: impl ToString, bit: usize, len: usize) -> Self {
        assert!(bit <= self.bit);
        self.list.push(Entry::read_only(name, bit, len));
        self
    }

    pub fn trigger(mut self, name: impl ToString) -> Self {
        self.list.push(Entry::trigger(name));
        self
    }
}

impl Module {
    pub fn axi_lite_slave(
        mut self,
        clk: impl ToString + Clone,
        rst: impl ToString + Clone,
        map: AXILiteSlave,
    ) -> Self {
        // Allocate Registors
        let (aloc, size) = {
            let mut addr = 0;
            let mut aloc = vec![];
            for entry in &map.list {
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
            .input(&map.awaddr, addr_width)
            .input(&map.awvalid, 1)
            .output(&map.awready, 1)
            .input(&map.wdata, map.bit)
            .input(&map.wstrb, map.bit / 8)
            .input(&map.wvalid, 1)
            .output(&map.wready, 1)
            .output(&map.bresp, 2)
            .output(&map.bvalid, 1)
            .input(&map.bready, 1)
            .input(&map.araddr, addr_width)
            .input(&map.arvalid, 1)
            .output(&map.arready, 1)
            .output(&map.rdata, map.bit)
            .output(&map.rresp, 2)
            .output(&map.rvalid, 1)
            .input(&map.rready, 1);

        // Regs
        for entry in &map.list {
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
            let mut cases = Case::new(&map.awaddr);
            for entry in &aloc {
                if let Some(name) = &entry.write {
                    cases = cases.case(
                        &format!("{}", entry.addr),
                        Stmt::assign(name, &format!("{}{}", map.wdata, range(entry.bit, 0))),
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
                    &format!("{} && {}", map.wvalid, map.awvalid),
                    Stmt::begin().case(case).end(),
                )
                .end(),
        );

        // Read Logic
        let case = {
            let mut cases = Case::new(&map.araddr);
            for entry in &aloc {
                if let Some(name) = &entry.read {
                    cases = cases.case(
                        &format!("{}", entry.addr),
                        Stmt::assign(&format!("{}{}", map.rdata, range(entry.bit, 0)), name),
                    );
                }
            }
            cases.default(Stmt::assign(&map.rdata, "0"))
        };
        self = self.sync_ff(
            clk.clone(),
            rst.clone(),
            Stmt::assign(&map.rdata, "0"),
            Stmt::begin()
                .r#if(&map.arvalid, Stmt::begin().case(case).end())
                .end(),
        );

        // AXI Lite Protocol
        self = self.sync_ff(
            clk,
            rst,
            Stmt::begin()
                .assign(&map.awready, "0")
                .assign(&map.wready, "0")
                .assign(&map.bvalid, "0")
                .assign(&map.arready, "0")
                .assign(&map.rvalid, "0")
                .assign(&map.bresp, "0")
                .assign(&map.rresp, "0")
                .end(),
            Stmt::begin()
                .assign(
                    &map.awready,
                    &format!("{} && !{}", map.awvalid, map.awready),
                )
                .assign(&map.wready, &format!("{} && !{}", map.wvalid, map.wready))
                .assign(
                    &map.bvalid,
                    &format!("{} && {} && !{}", map.awready, map.wready, map.bvalid),
                )
                .assign(
                    &map.arready,
                    &format!("{} && !{}", map.arvalid, map.arready),
                )
                .assign(&map.rvalid, &format!("{} && !{}", map.arvalid, map.arready))
                .r#if(
                    &format!("{} && {}", map.bvalid, map.bready),
                    Stmt::assign(&map.bvalid, "0"),
                )
                .r#if(
                    &format!("{} && {}", map.rvalid, map.rready),
                    Stmt::assign(&map.rvalid, "0"),
                )
                .end(),
        );

        self
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Entry {
    ReadWrite {
        name: String,
        bit: usize,
        len: usize,
    },
    ReadOnly {
        name: String,
        bit: usize,
        len: usize,
    },
    Trigger {
        name: String,
    },
}

impl Entry {
    fn read_only(name: impl ToString, bit: usize, len: usize) -> Self {
        assert!(0 < bit);
        assert!(0 < len);
        Self::ReadOnly {
            name: name.to_string(),
            bit,
            len,
        }
    }
    fn read_write(name: impl ToString, bit: usize, len: usize) -> Self {
        assert!(0 < bit);
        assert!(0 < len);
        Self::ReadWrite {
            name: name.to_string(),
            bit,
            len,
        }
    }
    fn trigger(name: impl ToString) -> Self {
        Self::Trigger {
            name: name.to_string(),
        }
    }

    fn allocate(&self, addr: usize, idx: usize) -> Allocated {
        match self {
            Self::ReadWrite { name, bit, len } => Allocated {
                read: Some(format!("{}{}", name, sel(idx, *len))),
                write: Some(format!("{}{}", name, sel(idx, *len))),
                bit: *bit,
                addr,
            },
            Self::ReadOnly { name, bit, len } => Allocated {
                read: Some(format!("{}{}", name, sel(idx, *len))),
                write: None,
                bit: *bit,
                addr,
            },
            Self::Trigger { name } => Allocated {
                read: Some(format!("{}_resp", name)),
                write: Some(format!("{}_trig", name)),
                bit: 1,
                addr,
            },
        }
    }
    fn len(&self) -> usize {
        match self {
            Self::ReadWrite { len, .. } => *len,
            Self::ReadOnly { len, .. } => *len,
            Self::Trigger { .. } => 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Allocated {
    read: Option<String>,
    write: Option<String>,
    addr: usize,
    bit: usize,
}
