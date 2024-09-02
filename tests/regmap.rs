use ruverta::{module::Module, regmap::RegMap};
use std::{fs, path::PathBuf};

#[test]
fn test_regmap() {
    let name = "test_regmap";
    let m = Module::new(name).input("clk", 1).input("rstn", 1).regmap(
        "clk",
        "rstn",
        RegMap::new("cbus", 32)
            .read_write("csr_rw0", 8, 1)
            .read_write("csr_rw1", 8, 1)
            .read_only("csr_ro", 8, 1)
            .trigger("csr_tw"),
    );
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", name));
    fs::write(path, s).unwrap();
}
