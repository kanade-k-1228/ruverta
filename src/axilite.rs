use crate::{
    module::Module,
    stmt::{Case, Stmt},
    util::{clog2, range, sel},
};

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AXILite {
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

impl AXILite {
    pub fn new(name: &str, bit: usize) -> Self {
        assert!(bit == 32 || bit == 64);
        Self {
            name: name.to_string(),
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

    pub fn read_write(mut self, name: &str, bit: usize, len: usize) -> Self {
        assert!(bit <= self.bit);
        self.list.push(Entry::read_write(name, bit, len));
        self
    }

    pub fn read_only(mut self, name: &str, bit: usize, len: usize) -> Self {
        assert!(bit <= self.bit);
        self.list.push(Entry::read_only(name, bit, len));
        self
    }

    pub fn trigger(mut self, name: &str) -> Self {
        self.list.push(Entry::trigger(name));
        self
    }
}

impl Module {
    pub fn axilite(mut self, clk: &str, rst: &str, axilite: AXILite) -> Self {
        // Allocate Registors
        let (aloc, size) = {
            let mut addr = 0;
            let mut aloc = vec![];
            for entry in &axilite.list {
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
            .input(&axilite.awaddr, addr_width)
            .input(&axilite.awvalid, 1)
            .output(&axilite.awready, 1)
            .input(&axilite.wdata, axilite.bit)
            .input(&axilite.wstrb, axilite.bit / 8)
            .input(&axilite.wvalid, 1)
            .output(&axilite.wready, 1)
            .output(&axilite.bresp, 2)
            .output(&axilite.bvalid, 1)
            .input(&axilite.bready, 1)
            .input(&axilite.araddr, addr_width)
            .input(&axilite.arvalid, 1)
            .output(&axilite.arready, 1)
            .output(&axilite.rdata, axilite.bit)
            .output(&axilite.rresp, 2)
            .output(&axilite.rvalid, 1)
            .input(&axilite.rready, 1);

        // Regs
        for entry in &axilite.list {
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
            let mut cases = Case::new(&axilite.awaddr);
            for entry in &aloc {
                if let Some(name) = &entry.write {
                    cases = cases.case(
                        &format!("{}", entry.addr),
                        Stmt::assign(&name, &format!("{}{}", axilite.wdata, range(entry.bit, 0))),
                    );
                }
            }
            cases.default(Stmt::empty())
        };
        self = self.sync_ff(
            clk,
            rst,
            init,
            Stmt::begin()
                .r#if(
                    &format!("{} && {}", axilite.wvalid, axilite.awvalid),
                    Stmt::begin().case(case).end(),
                )
                .end(),
        );

        // Read Logic
        let case = {
            let mut cases = Case::new(&axilite.araddr);
            for entry in &aloc {
                if let Some(name) = &entry.read {
                    cases = cases.case(
                        &format!("{}", entry.addr),
                        Stmt::assign(&format!("{}{}", axilite.rdata, range(entry.bit, 0)), &name),
                    );
                }
            }
            cases.default(Stmt::assign(&axilite.rdata, "0"))
        };
        self = self.sync_ff(
            clk,
            rst,
            Stmt::assign(&axilite.rdata, "0"),
            Stmt::begin()
                .r#if(&axilite.arvalid, Stmt::begin().case(case).end())
                .end(),
        );

        // AXI Lite Protocol
        self = self.sync_ff(
            clk,
            rst,
            Stmt::begin()
                .assign(&axilite.awready, "0")
                .assign(&axilite.wready, "0")
                .assign(&axilite.bvalid, "0")
                .assign(&axilite.arready, "0")
                .assign(&axilite.rvalid, "0")
                .assign(&axilite.bresp, "0")
                .assign(&axilite.rresp, "0")
                .end(),
            Stmt::begin()
                .assign(
                    &axilite.awready,
                    &format!("{} && !{}", axilite.awvalid, axilite.awready),
                )
                .assign(
                    &axilite.wready,
                    &format!("{} && !{}", axilite.wvalid, axilite.wready),
                )
                .assign(
                    &axilite.bvalid,
                    &format!(
                        "{} && {} && !{}",
                        axilite.awready, axilite.wready, axilite.bvalid
                    ),
                )
                .assign(
                    &axilite.arready,
                    &format!("{} && !{}", axilite.arvalid, axilite.arready),
                )
                .assign(
                    &axilite.rvalid,
                    &format!("{} && !{}", axilite.arvalid, axilite.arready),
                )
                .r#if(
                    &format!("{} && {}", axilite.bvalid, axilite.bready),
                    Stmt::assign(&axilite.bvalid, "0"),
                )
                .r#if(
                    &format!("{} && {}", axilite.rvalid, axilite.rready),
                    Stmt::assign(&axilite.rvalid, "0"),
                )
                .end(),
        );

        self
    }

    // pub fn regio(mut self, config: &RegMap) -> Self {
    //     for reg in &config.list {
    //         self = match reg.ty {
    //             RegType::ReadWrite => self.output(&reg.name, reg.bit),
    //             RegType::ReadOnly => self.input(&reg.name, reg.bit),
    //             RegType::Trigger => self
    //                 .output(&format!("{}_trig", reg.name), reg.bit)
    //                 .input(&format!("{}_resp", reg.name), reg.bit),
    //         };
    //     }
    //     self
    // }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
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
    fn read_only(name: &str, bit: usize, len: usize) -> Self {
        assert!(0 < bit);
        assert!(0 < len);
        Self::ReadOnly {
            name: name.to_string(),
            bit,
            len,
        }
    }
    fn read_write(name: &str, bit: usize, len: usize) -> Self {
        assert!(0 < bit);
        assert!(0 < len);
        Self::ReadWrite {
            name: name.to_string(),
            bit,
            len,
        }
    }
    fn trigger(name: &str) -> Self {
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

#[derive(Debug)]
struct Allocated {
    read: Option<String>,
    write: Option<String>,
    addr: usize,
    bit: usize,
}
