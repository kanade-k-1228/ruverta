use crate::{
    module::Module,
    stmt::{Case, Stmt},
    util::clog2,
};

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct RegMap {
    name: String,
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

impl RegMap {
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
        self.list
            .push(Entry::new(RegType::ReadWrite, name, bit, len));
        self
    }

    pub fn read_only(mut self, name: &str, bit: usize, len: usize) -> Self {
        assert!(bit <= self.bit);
        self.list
            .push(Entry::new(RegType::ReadOnly, name, bit, len));
        self
    }

    pub fn trigger(mut self, name: &str) -> Self {
        self.list.push(Entry::new(RegType::Trigger, name, 1, 1));
        self
    }
}

impl Module {
    pub fn regmap(mut self, clk: &str, rst: &str, regmap: RegMap) -> Self {
        // Allocate Registors
        let alocated = regmap
            .list
            .iter()
            .scan(0_usize, |addr, entry| {
                let begin = *addr;
                let end = begin + entry.len - 1;
                *addr = *addr + entry.len;
                Some((begin, end, entry))
            })
            .collect::<Vec<_>>();
        let addr_width = {
            let regcnt = alocated.last().map(|(_, last, _)| *last).unwrap_or(64);
            clog2(regcnt).unwrap_or(6)
        };

        // IO Port
        self = self
            .input(&regmap.awaddr, addr_width)
            .input(&regmap.awvalid, 1)
            .output(&regmap.awready, 1)
            .input(&regmap.wdata, regmap.bit)
            .input(&regmap.wstrb, regmap.bit / 8)
            .input(&regmap.wvalid, 1)
            .output(&regmap.wready, 1)
            .output(&regmap.bresp, 2)
            .output(&regmap.bvalid, 1)
            .input(&regmap.bready, 1)
            .input(&regmap.araddr, addr_width)
            .input(&regmap.arvalid, 1)
            .output(&regmap.arready, 1)
            .output(&regmap.rdata, regmap.bit)
            .output(&regmap.rresp, 2)
            .output(&regmap.rvalid, 1)
            .input(&regmap.rready, 1);

        // Regs
        for (_, _, entry) in &alocated {
            self = match entry.ty {
                RegType::ReadWrite => self.logic(&entry.name, entry.bit, entry.len),
                RegType::ReadOnly => self.logic(&entry.name, entry.bit, entry.len),
                RegType::Trigger => self.logic(&entry.rname(), entry.bit, entry.len).logic(
                    &entry.wname(),
                    entry.bit,
                    entry.len,
                ),
            };
        }

        // Write Logic
        self = self.sync_ff(
            clk,
            rst,
            {
                alocated
                    .iter()
                    .fold(Stmt::begin(), |stmt, (_, _, entry)| {
                        stmt.assign(&entry.wname(), "0")
                    })
                    .end()
            },
            Stmt::begin()
                .r#if(
                    &format!("{} && {}", regmap.wvalid, regmap.awvalid),
                    Stmt::begin()
                        .case(alocated.iter().fold(
                            Case::new(&regmap.awaddr),
                            |case, (addr, _, entry)| {
                                case.case(
                                    &format!("{}", addr),
                                    Stmt::assign(&entry.wname(), &regmap.wdata),
                                )
                            },
                        ))
                        .end(),
                )
                .end(),
        );

        // Read Logic
        self = self.sync_ff(
            clk,
            rst,
            Stmt::assign(&regmap.rdata, "0"),
            Stmt::begin()
                .r#if(
                    &regmap.arvalid,
                    Stmt::begin()
                        .case({
                            let cases = alocated.iter().fold(
                                Case::new(&regmap.araddr),
                                |case, (addr, _, entry)| {
                                    case.case(
                                        &format!("{}", addr),
                                        Stmt::assign(&regmap.rdata, &entry.rname()),
                                    )
                                },
                            );
                            cases
                        })
                        .end(),
                )
                .end(),
        );

        // AXI Lite Protocol
        self = self.sync_ff(
            clk,
            rst,
            Stmt::begin()
                .assign(&regmap.awready, "0")
                .assign(&regmap.wready, "0")
                .assign(&regmap.bvalid, "0")
                .assign(&regmap.arready, "0")
                .assign(&regmap.rvalid, "0")
                .assign(&regmap.bresp, "0")
                .assign(&regmap.rresp, "0")
                .end(),
            Stmt::begin()
                .assign(
                    &regmap.awready,
                    &format!("{} && !{}", regmap.awvalid, regmap.awready),
                )
                .assign(
                    &regmap.wready,
                    &format!("{} && !{}", regmap.wvalid, regmap.wready),
                )
                .assign(
                    &regmap.bvalid,
                    &format!(
                        "{} && {} && !{}",
                        regmap.awready, regmap.wready, regmap.bvalid
                    ),
                )
                .assign(
                    &regmap.arready,
                    &format!("{} && !{}", regmap.arvalid, regmap.arready),
                )
                .assign(
                    &regmap.rvalid,
                    &format!("{} && !{}", regmap.arvalid, regmap.arready),
                )
                .r#if(
                    &format!("{} && {}", regmap.bvalid, regmap.bready),
                    Stmt::assign(&regmap.bvalid, "0"),
                )
                .r#if(
                    &format!("{} && {}", regmap.rvalid, regmap.rready),
                    Stmt::assign(&regmap.rvalid, "0"),
                )
                .end(),
        );

        self
    }

    pub fn regio(mut self, config: &RegMap) -> Self {
        for reg in &config.list {
            self = match reg.ty {
                RegType::ReadWrite => self.output(&reg.name, reg.bit),
                RegType::ReadOnly => self.input(&reg.name, reg.bit),
                RegType::Trigger => self
                    .output(&format!("{}_trig", reg.name), reg.bit)
                    .input(&format!("{}_resp", reg.name), reg.bit),
            };
        }
        self
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct Entry {
    ty: RegType,
    name: String,
    bit: usize,
    len: usize,
}

#[derive(Debug)]
enum RegType {
    ReadWrite,
    ReadOnly,
    Trigger,
}

impl Entry {
    fn new(ty: RegType, name: &str, bit: usize, len: usize) -> Self {
        assert!(0 < bit);
        assert!(0 < len);
        Self {
            ty,
            name: name.to_string(),
            bit,
            len,
        }
    }

    fn wname(&self) -> String {
        match self.ty {
            RegType::ReadWrite => self.name.clone(),
            RegType::ReadOnly => self.name.clone(),
            RegType::Trigger => format!("{}_trig", self.name),
        }
    }
    fn rname(&self) -> String {
        match self.ty {
            RegType::ReadWrite => self.name.clone(),
            RegType::ReadOnly => self.name.clone(),
            RegType::Trigger => format!("{}_resp", self.name),
        }
    }
}
