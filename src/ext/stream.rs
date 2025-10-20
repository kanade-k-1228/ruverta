use crate::module::{Extension, Module};

#[derive(Debug, Clone)]
pub struct Stream {
    name: String,
    bit: usize,
}

impl Stream {
    pub fn new(name: impl ToString, bit: usize) -> Self {
        Self {
            name: name.to_string(),
            bit,
        }
    }
    fn data(&self) -> String {
        format!("{}_data", self.name)
    }
    fn valid(&self) -> String {
        format!("{}_valid", self.name)
    }
    fn ready(&self) -> String {
        format!("{}_ready", self.name)
    }
}

/// Stream configuration type
#[derive(Debug, Clone)]
pub enum StreamConfig {
    /// Stream slave interface (input data/valid, output ready)
    Slave(Stream),
    /// Stream master interface (output data/valid, input ready)
    Master(Stream),
    /// Stream wires for internal connections
    Wire(Stream),
}

impl Extension for StreamConfig {
    fn add(self, module: Module) -> Module {
        match self {
            StreamConfig::Slave(stream) => module
                .input(stream.data(), stream.bit)
                .input(stream.valid(), 1)
                .output(stream.ready(), 1),
            StreamConfig::Master(stream) => module
                .output(stream.data(), stream.bit)
                .output(stream.valid(), 1)
                .input(stream.ready(), 1),
            StreamConfig::Wire(stream) => module
                .logic(stream.data(), stream.bit, 1)
                .logic(stream.valid(), stream.bit, 1)
                .logic(stream.ready(), stream.bit, 1),
        }
    }
}
