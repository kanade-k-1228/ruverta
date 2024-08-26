use crate::module::Module;

pub trait Verilog {
    fn verilog(&self) -> String;
}

pub trait Builder {
    fn build(module: &mut Module, config: &Self);
}
