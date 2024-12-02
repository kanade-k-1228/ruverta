use super::{Entry, MMap};
use crate::{module::Module, util::clog2};

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PicoMaster {
    pub name: String,
    mmap: MMap,

    // Bus wire names
    ready: String,
    valid: String,
    addr: String,
    wstrb: String,
    wdata: String,
    rdata: String,
}

impl PicoMaster {
    pub fn new(name: impl ToString, mmap: MMap) -> Self {
        let name: String = name.to_string();
        Self {
            name: name.clone(),
            mmap,
            ready: format!("{name}_ready"),
            valid: format!("{name}_valid"),
            addr: format!("{name}_addr"),
            wstrb: format!("{name}_wstrb"),
            wdata: format!("{name}_wdata"),
            rdata: format!("{name}_rdata"),
        }
    }
}

impl Module {
    pub fn pico_master(
        mut self,
        clk: impl ToString + Clone,
        rst: impl ToString + Clone,
        bus: PicoMaster,
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
            .input(&bus.addr, addr_width)
            .input(&bus.wdata, bus.mmap.data_width)
            .input(&bus.wstrb, bus.mmap.data_width / 8)
            .output(&bus.rdata, bus.mmap.data_width);

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

        self
    }
}
