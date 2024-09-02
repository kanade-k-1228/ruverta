use ruverta::{
    module::{Module, Sens},
    stmt::Stmt,
};
use std::{fs, path::PathBuf};

#[test]
fn test_module() {
    let name = "test_fsm";
    let m = Module::new(name)
        .param("BIT", Some("8"))
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 8)
        .input("in1", 8)
        .output("out", 8)
        .always_comb(Stmt::assign("out", "in0 + in1"))
        .always_ff(
            Sens::new().posedge("clk"),
            Stmt::begin().assign("a", "b").end(),
        );
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", name));
    fs::write(path, s).unwrap();
}
