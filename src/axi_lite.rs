use crate::{
    module::Module,
    traits::{Builder, Verilog},
};

#[derive(Debug)]
struct AXILite {
    name: String,
    data_width: u32,
    list: Vec<Entry>,
}

impl AXILite {
    pub fn new(name: &str, width: u32) -> Self {
        assert!(width == 32 || width == 64);
        Self {
            name: name.to_string(),
            data_width: width,
            list: vec![],
        }
    }
    pub fn reg(&mut self, ty: Reg, name: &str, bit: u32) {
        assert!(bit < self.data_width);
        self.list.push(Entry::Reg(ty, name.to_string(), bit))
    }
    pub fn vec(&mut self, ty: Reg, name: &str, len: u32) {
        self.list.push(Entry::Vec(ty, name.to_string(), len))
    }
}

impl Builder for AXILite {
    fn build(module: &mut Module, config: &Self) {
        // Allocate Registors
        for entry in &config.list {
            println!("Create: {:?}", entry);
        }
        let addr_width = 7;

        // IO Port
        module.input(&format!("{}_awaddr", config.name), addr_width);
        module.input(&format!("{}_awvalid", config.name), 1);
        module.output(&format!("{}_awready", config.name), 1);
        module.input(&format!("{}_wdata", config.name), config.data_width);
        module.input(&format!("{}_wstrb", config.name), config.data_width / 8);
        module.input(&format!("{}_wvalid", config.name), 1);
        module.output(&format!("{}_wready", config.name), 1);
        module.output(&format!("{}_bresp", config.name), 2);
        module.output(&format!("{}_bvalid", config.name), 1);
        module.input(&format!("{}_bready", config.name), 1);
        module.input(&format!("{}_araddr", config.name), addr_width);
        module.input(&format!("{}_arvalid", config.name), 1);
        module.output(&format!("{}_arready", config.name), 1);
        module.output(&format!("{}_rdata", config.name), config.data_width);
        module.output(&format!("{}_rvalid", config.name), 1);
        module.input(&format!("{}_rready", config.name), 1);
    }
}

#[derive(Debug)]
enum Entry {
    Reg(Reg, String, u32),
    Vec(Reg, String, u32),
}

#[derive(Debug)]
enum Reg {
    ReadWrite,
    ReadOnly,
}

#[test]
fn test_axi_lite() {
    let mut module = Module::new("sample_mod");
    let mut axil = AXILite::new("cbus", 32);
    axil.reg(Reg::ReadWrite, "ctrl", 2);
    axil.vec(Reg::ReadOnly, "buf", 8);
    AXILite::build(&mut module, &axil);
    println!("{}", module.verilog())
}
