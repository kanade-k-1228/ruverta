use super::{MemMap, Reg};
use crate::module::Module;

impl Module {
    pub(in crate::bus) fn define_regs(mut self, mem: &MemMap) -> Self {
        for reg in &mem.regs {
            self = match reg {
                Reg::ReadWrite { name, bit, len } => self.logic(name, *bit, *len),
                Reg::ReadOnly { name, bit, len } => self.logic(name, *bit, *len),
                Reg::WriteOnly { name, bit, len } => self.logic(name, *bit, *len),
                Reg::Trigger { name } => {
                    self.logic(&format!("{name}_trig"), 1, 1)
                        .logic(&format!("{name}_resp"), 1, 1)
                }
            };
        }
        self
    }
}
