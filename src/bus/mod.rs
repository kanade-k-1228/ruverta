mod axi_lite;
mod common;
#[cfg(feature = "unstable")]
mod pico;

pub use axi_lite::AXILiteSlave;
#[cfg(feature = "unstable")]
pub use pico::PicoSlave;

// ----------------------------------------------------------------------------

use crate::util::sel;

#[derive(Debug, Clone)]
pub struct RegList {
    regs: Vec<Reg>,
}

#[derive(Debug, Clone)]
enum Reg {
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
    WriteOnly {
        name: String,
        bit: usize,
        len: usize,
    },
    Trigger {
        name: String,
    },
}

impl Reg {
    fn len(&self) -> usize {
        match self {
            Self::ReadWrite { len, .. } => *len,
            Self::ReadOnly { len, .. } => *len,
            Self::WriteOnly { len, .. } => *len,
            Self::Trigger { .. } => 1,
        }
    }
}

impl RegList {
    pub fn new() -> Self {
        Self { regs: vec![] }
    }
    pub fn read_write(mut self, name: impl ToString, bit: usize, len: usize) -> Self {
        assert!(0 < bit && 0 < len);
        self.regs.push(Reg::ReadWrite {
            name: name.to_string(),
            bit,
            len,
        });
        self
    }
    pub fn read_only(mut self, name: impl ToString, bit: usize, len: usize) -> Self {
        assert!(0 < bit && 0 < len);
        self.regs.push(Reg::ReadOnly {
            name: name.to_string(),
            bit,
            len,
        });
        self
    }
    pub fn trigger(mut self, name: impl ToString) -> Self {
        self.regs.push(Reg::Trigger {
            name: name.to_string(),
        });
        self
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct MemMap {
    data_bit: usize,
    addr_bit: usize,
    regs: Vec<Reg>,
    map: Vec<Entry>,
}

#[derive(Debug, Clone)]
struct Entry {
    addr: usize,
    bit: usize,
    read: Option<String>,
    write: Option<String>,
}

impl RegList {
    pub fn allocate_greedy(self, data_bit: usize, addr_bit: usize) -> MemMap {
        let (list, _) = {
            let mut addr = 0;
            let mut list = vec![];
            for entry in &self.regs {
                for idx in 0..entry.len() {
                    list.push(match entry {
                        Reg::ReadWrite { name, bit, len } => Entry {
                            read: Some(format!("{}{}", name, sel(idx, *len))),
                            write: Some(format!("{}{}", name, sel(idx, *len))),
                            bit: *bit,
                            addr,
                        },
                        Reg::ReadOnly { name, bit, len } => Entry {
                            read: Some(format!("{}{}", name, sel(idx, *len))),
                            write: None,
                            bit: *bit,
                            addr,
                        },
                        Reg::WriteOnly { name, bit, len } => Entry {
                            read: None,
                            write: Some(format!("{}{}", name, sel(idx, *len))),
                            bit: *bit,
                            addr,
                        },
                        Reg::Trigger { name } => Entry {
                            read: Some(format!("{}_resp", name)),
                            write: Some(format!("{}_trig", name)),
                            bit: 1,
                            addr,
                        },
                    });
                    addr += 1;
                }
            }
            (list, addr)
        };
        return MemMap {
            data_bit,
            addr_bit,
            regs: self.regs,
            map: list,
        };
    }
}
