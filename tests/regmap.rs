use ruverta::{module::Module, regmap::RegMap};
use std::{fs, path::PathBuf};

const NAME: &str = "regmap";

#[test]
fn test_regmap() {
    let m = Module::new(NAME).input("clk", 1).input("rstn", 1).regmap(
        "clk",
        "rstn",
        RegMap::new("cbus", 32)
            .read_write("csr_rw", 8, 2)
            .read_only("csr_ro", 8, 1)
            .trigger("csr_tw"),
    );
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", NAME));
    fs::write(path, s).unwrap();
}
