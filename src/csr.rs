use crate::module::Module;

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct RegMap {
    name: String,
    bit: u32,
    list: Vec<Entry>,
}

impl RegMap {
    pub fn new(name: &str, bit: u32) -> Self {
        assert!(bit == 32 || bit == 64);
        Self {
            name: name.to_string(),
            bit,
            list: vec![],
        }
    }

    pub fn read_write(mut self, name: &str, bit: u32, len: u32) -> Self {
        assert!(bit <= self.bit);
        self.list
            .push(Entry::new(RegType::ReadWrite, name, bit, len));
        self
    }

    pub fn read_only(mut self, name: &str, bit: u32, len: u32) -> Self {
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
    pub fn regmap(mut self, config: &RegMap) -> Self {
        // Allocate Registors
        let addr_width = 7;

        // IO Port
        self = self
            .input(&format!("{}_awaddr", config.name), addr_width)
            .input(&format!("{}_awvalid", config.name), 1)
            .output(&format!("{}_awready", config.name), 1)
            .input(&format!("{}_wdata", config.name), config.bit)
            .input(&format!("{}_wstrb", config.name), config.bit / 8)
            .input(&format!("{}_wvalid", config.name), 1)
            .output(&format!("{}_wready", config.name), 1)
            .output(&format!("{}_bresp", config.name), 2)
            .output(&format!("{}_bvalid", config.name), 1)
            .input(&format!("{}_bready", config.name), 1)
            .input(&format!("{}_araddr", config.name), addr_width)
            .input(&format!("{}_arvalid", config.name), 1)
            .output(&format!("{}_arready", config.name), 1)
            .output(&format!("{}_rdata", config.name), config.bit)
            .output(&format!("{}_rvalid", config.name), 1)
            .input(&format!("{}_rready", config.name), 1);

        // Regs
        for reg in &config.list {
            self = match reg.ty {
                RegType::ReadWrite => self.logic(&reg.rname(), reg.bit, reg.len),
                RegType::ReadOnly => self.logic(&reg.rname(), reg.bit, reg.len),
                RegType::Trigger => self
                    .logic(&format!("{}_trig", reg.rname()), reg.bit, reg.len)
                    .logic(&format!("{}_resp", reg.rname()), reg.bit, reg.len),
            };
        }
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
    bit: u32,
    len: u32,
}

#[derive(Debug)]
enum RegType {
    ReadWrite,
    ReadOnly,
    Trigger,
}

impl Entry {
    fn new(ty: RegType, name: &str, bit: u32, len: u32) -> Self {
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

// ----------------------------------------------------------------------------
