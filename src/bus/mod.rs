pub mod axi_lite_slave;
pub mod pico_master;
pub mod pico_slave;

// ----------------------------------------------------------------------------
use crate::util::sel;

#[derive(Debug, Clone)]
pub enum Entry {
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
    Trigger {
        name: String,
    },
}

impl Entry {
    fn read_only(name: impl ToString, bit: usize, len: usize) -> Self {
        assert!(0 < bit);
        assert!(0 < len);
        Self::ReadOnly {
            name: name.to_string(),
            bit,
            len,
        }
    }
    fn read_write(name: impl ToString, bit: usize, len: usize) -> Self {
        assert!(0 < bit);
        assert!(0 < len);
        Self::ReadWrite {
            name: name.to_string(),
            bit,
            len,
        }
    }
    fn trigger(name: impl ToString) -> Self {
        Self::Trigger {
            name: name.to_string(),
        }
    }

    fn allocate(&self, addr: usize, idx: usize) -> Allocated {
        match self {
            Self::ReadWrite { name, bit, len } => Allocated {
                read: Some(format!("{}{}", name, sel(idx, *len))),
                write: Some(format!("{}{}", name, sel(idx, *len))),
                bit: *bit,
                addr,
            },
            Self::ReadOnly { name, bit, len } => Allocated {
                read: Some(format!("{}{}", name, sel(idx, *len))),
                write: None,
                bit: *bit,
                addr,
            },
            Self::Trigger { name } => Allocated {
                read: Some(format!("{}_resp", name)),
                write: Some(format!("{}_trig", name)),
                bit: 1,
                addr,
            },
        }
    }
    fn len(&self) -> usize {
        match self {
            Self::ReadWrite { len, .. } => *len,
            Self::ReadOnly { len, .. } => *len,
            Self::Trigger { .. } => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Allocated {
    read: Option<String>,
    write: Option<String>,
    addr: usize,
    bit: usize,
}

#[derive(Debug, Clone)]
pub struct MMap {
    pub(in crate::bus) data_width: usize,
    pub(in crate::bus) addr_width: usize,
    pub(in crate::bus) list: Vec<Entry>,
}

impl MMap {
    pub fn new(data_width: usize, addr_width: usize) -> Self {
        Self {
            data_width,
            addr_width,
            list: vec![],
        }
    }

    pub fn read_write(mut self, name: impl ToString, bit: usize, len: usize) -> Self {
        assert!(bit <= self.data_width);
        self.list.push(Entry::read_write(name, bit, len));
        self
    }

    pub fn read_only(mut self, name: impl ToString, bit: usize, len: usize) -> Self {
        assert!(bit <= self.data_width);
        self.list.push(Entry::read_only(name, bit, len));
        self
    }

    pub fn trigger(mut self, name: impl ToString) -> Self {
        self.list.push(Entry::trigger(name));
        self
    }
}
