use ruverta::{fifo::FIFO, module::Module};
use std::{fs, path::PathBuf};

const NAME: &str = "fifo";

#[test]
fn test_fifo() {
    let m = Module::new(NAME).fifo(FIFO::new("rx", 8, 32));
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", NAME));
    fs::write(path, s).unwrap();
}
