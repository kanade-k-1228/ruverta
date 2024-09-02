use crate::{module::Module, util::clog2};

#[derive(Debug)]
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
    pub fn new(name: &str, bit: usize, len: usize) -> Self {
        Self {
            name: name.to_string(),
            bit,
            len,
            addr_width: clog2(len).unwrap_or(1),
            buf: format!("{name}_buf"),
            rptr: format!("{name}_rptr"),
            wptr: format!("{name}_wptr"),
        }
    }
}

impl Module {
    pub fn fifo(mut self, fifo: FIFO) -> Self {
        self = self
            .logic(&fifo.buf, fifo.bit, fifo.len)
            .logic(&fifo.rptr, fifo.addr_width, 1)
            .logic(&fifo.wptr, fifo.addr_width, 1);
        self
    }
}
