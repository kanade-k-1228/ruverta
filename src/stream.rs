use crate::module::Module;

#[derive(Debug)]
pub struct Stream {
    name: String,
    bit: usize,
}

impl Stream {
    pub fn new(name: &str, bit: usize) -> Self {
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

impl Module {
    pub fn stream_slave(self, stream: Stream) -> Self {
        self.input(&stream.data(), stream.bit)
            .input(&stream.valid(), 1)
            .output(&stream.ready(), 1)
    }
    pub fn stream_master(self, stream: Stream) -> Self {
        self.output(&stream.data(), stream.bit)
            .output(&stream.valid(), 1)
            .input(&stream.ready(), 1)
    }
    pub fn stream_wire(self, stream: Stream) -> Self {
        self.logic(&stream.data(), stream.bit, 1)
            .logic(&stream.valid(), stream.bit, 1)
            .logic(&stream.ready(), stream.bit, 1)
    }
}
