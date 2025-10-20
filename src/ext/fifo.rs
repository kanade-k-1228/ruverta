use crate::{
    module::{Extension, Module},
    util::clog2,
};

#[derive(Debug, Clone)]
pub struct FIFO {
    pub name: String,
    bit: usize,
    len: usize,
    addr_width: usize,
    buf: String,
    rptr: String,
    wptr: String,
}

impl FIFO {
    pub fn new(name: impl ToString, bit: usize, len: usize) -> Self {
        let name: String = name.to_string();
        Self {
            name: name.clone(),
            bit,
            len,
            addr_width: clog2(len).unwrap_or(1),
            buf: format!("{name}_buf"),
            rptr: format!("{name}_rptr"),
            wptr: format!("{name}_wptr"),
        }
    }
}

impl Extension for FIFO {
    fn add(self, module: Module) -> Module {
        module
            .logic(&self.buf, self.bit, self.len)
            .logic(&self.rptr, self.addr_width, 1)
            .logic(&self.wptr, self.addr_width, 1)
    }
}
