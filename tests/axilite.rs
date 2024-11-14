use ruverta::{axilite::AXILite, module::Module};
use std::{fs, path::PathBuf};

const NAME: &str = "axilite";

#[test]
fn test_axilite() {
    let m = Module::new(NAME).input("clk", 1).input("rstn", 1).axilite(
        "clk",
        "rstn",
        AXILite::new("cbus", 32)
            .read_write("csr_rw", 8, 2)
            .read_only("csr_ro", 8, 1)
            .trigger("csr_tw"),
    );
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", NAME));
    fs::write(path, s).unwrap();
}
