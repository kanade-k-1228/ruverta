use crate::{dff::Dff, module::Module, stmt::Stmt};

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct RegMap {
    name: String,
    bit: usize,
    list: Vec<Entry>,
}

impl RegMap {
    pub fn new(name: &str, bit: usize) -> Self {
        assert!(bit == 32 || bit == 64);
        Self {
            name: name.to_string(),
            bit,
            list: vec![],
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
    pub fn regmap(mut self, regmap: RegMap) -> Self {
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
        let addr_width = alocated.last().map(|(_, last, _)| *last).unwrap_or(7);

        // IO Port
        self = self
            .input(&format!("{}_awaddr", regmap.name), addr_width)
            .input(&format!("{}_awvalid", regmap.name), 1)
            .output(&format!("{}_awready", regmap.name), 1)
            .input(&format!("{}_wdata", regmap.name), regmap.bit)
            .input(&format!("{}_wstrb", regmap.name), regmap.bit / 8)
            .input(&format!("{}_wvalid", regmap.name), 1)
            .output(&format!("{}_wready", regmap.name), 1)
            .output(&format!("{}_bresp", regmap.name), 2)
            .output(&format!("{}_bvalid", regmap.name), 1)
            .input(&format!("{}_bready", regmap.name), 1)
            .input(&format!("{}_araddr", regmap.name), addr_width)
            .input(&format!("{}_arvalid", regmap.name), 1)
            .output(&format!("{}_arready", regmap.name), 1)
            .output(&format!("{}_rdata", regmap.name), regmap.bit)
            .output(&format!("{}_rvalid", regmap.name), 1)
            .input(&format!("{}_rready", regmap.name), 1);

        // Regs
        for (_, _, entry) in &alocated {
            self = match entry.ty {
                RegType::ReadWrite => self.logic(&entry.rname(), entry.bit, entry.len),
                RegType::ReadOnly => self.logic(&entry.rname(), entry.bit, entry.len),
                RegType::Trigger => self
                    .logic(&format!("{}_trig", entry.rname()), entry.bit, entry.len)
                    .logic(&format!("{}_resp", entry.rname()), entry.bit, entry.len),
            };
        }

        // Write Addr
        self = self.logic("aw_en", 1, 1).sync_ff(
            "clk",
            "rstn",
            Stmt::begin()
                .assign("axi_awready", "0")
                .assign("aw_en", "1'b1")
                .end(),
            Stmt::cond()
                .r#if(
                    "~axi_awready && S_AXI_AWVALID && S_AXI_WVALID && aw_en",
                    Stmt::begin()
                        .assign("axi_awready", "1")
                        .assign("aw_en", "0")
                        .end(),
                )
                .r#if(
                    "S_AXI_BREADY && axi_bvalid",
                    Stmt::begin()
                        .assign("axi_awready", "0")
                        .assign("aw_en", "1")
                        .end(),
                )
                .r#else(Stmt::begin().assign("axi_awready", "0").end()),
        );

        self = self.sync_ff(
            "clk",
            "rstn",
            Stmt::assign("axi_awaddr", "0"),
            Stmt::cond()
                .r#if(
                    "~axi_awready && S_AXI_AWVALID && S_AXI_WVALID && aw_en",
                    Stmt::assign("axi_awaddr", "S"),
                )
                .r#else(Stmt::empty()),
        );

        // Write Response
        self = self.sync_ff(
            "clk",
            "rstn",
            Stmt::begin()
                .assign("axi_bvalid", "0")
                .assign("axi_bresp", "2'b0")
                .end(),
            Stmt::cond()
                .r#if(
                    "axi_awready && S_AXI_AWVALID && ~axi_bvalid && axi_wready && S_AXI_WVALID",
                    Stmt::begin()
                        .assign("axi_bvalid", "1")
                        .assign("axi_bresp", "0")
                        .end(),
                )
                .r#if(
                    "S_AXI_BREADY && axi_bvalid",
                    Stmt::assign("axi_bvalid", "0"),
                )
                .r#else(Stmt::empty()),
        );

        // Read Address
        self = self.sync_ff(
            "clk",
            "rstn",
            Stmt::begin()
                .assign("axi_arready", "0")
                .assign("axi_araddr", "32'b0")
                .end(),
            Stmt::cond()
                .r#if(
                    "~axi_arready && S_AXI_ARVALID",
                    Stmt::begin()
                        .assign("axi_arready", "1")
                        .assign("axi_araddr", "S_AXI_ARADDR")
                        .end(),
                )
                .r#else(Stmt::assign("axi_arready", "0")),
        );

        // Read Data
        self = self.sync_ff(
            "clk",
            "rstn",
            Stmt::begin()
                .assign("axi_rvalid", "0")
                .assign("axi_rresp", "0")
                .end(),
            Stmt::cond()
                .r#if(
                    "axi_arready && S_AXI_ARVALID && ~axi_rvalid",
                    Stmt::begin()
                        .assign("axi_rvalid", "1")
                        .assign("axi_rresp", "2'b0")
                        .end(),
                )
                .r#if(
                    "axi_rvalid && S_AXI_RREADY",
                    Stmt::assign("axi_rvalid", "0"),
                )
                .r#else(Stmt::begin().end()),
        );

        self
    }

    pub fn regio(mut self, config: &RegMap) -> Self {
        for reg in &config.list {
            self = match reg.ty {
                RegType::ReadWrite => self.output(&reg.rname(), reg.bit),
                RegType::ReadOnly => self.input(&reg.rname(), reg.bit),
                RegType::Trigger => self
                    .output(&format!("{}_trig", reg.rname()), reg.bit)
                    .input(&format!("{}_resp", reg.rname()), reg.bit),
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
    fn rname(&self) -> String {
        match self.ty {
            RegType::ReadWrite => format!("rw_{}", self.name),
            RegType::ReadOnly => format!("ro_{}", self.name),
            RegType::Trigger => format!("tw_{}", self.name),
        }
    }
}
