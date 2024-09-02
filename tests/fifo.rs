use ruverta::{fifo::FIFO, module::Module};
use std::{fs, path::PathBuf};

#[test]
fn test_fifo() {
    let name = "test_fifo";
    let m = Module::new(name).fifo(FIFO::new("rx", 8, 32));
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", name));
    fs::write(path, s).unwrap();
}
