use ruverta::{module::Module, stmt::Stmt};
use std::{fs, path::PathBuf};

const NAME: &str = "dff";

#[test]
fn test_dff() {
    let m = Module::new(NAME)
        .param("BIT", Some("8"))
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 8)
        .input("in1", 8)
        .output("out", 8)
        .sync_ff(
            "clk",
            "rstn",
            Stmt::begin().assign("out", "0").end(),
            Stmt::begin().assign("out", "in0 + in1").end(),
        );
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", NAME));
    fs::write(path, s).unwrap();
}
