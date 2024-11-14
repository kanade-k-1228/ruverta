use ruverta::{axi_lite_slave::AXILiteSlave, module::Module};
use std::{fs, path::PathBuf};

const NAME: &str = "axi_lite_slave";

#[test]
fn test_axi_lite_slave() {
    let m = Module::new(NAME)
        .input("clk", 1)
        .input("rstn", 1)
        .axi_lite_slave(
            "clk",
            "rstn",
            AXILiteSlave::new("cbus", 32)
                .read_write("csr_rw", 8, 2)
                .read_only("csr_ro", 8, 1)
                .trigger("csr_tw"),
        );
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", NAME));
    fs::write(path, s).unwrap();
}
