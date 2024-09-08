use ruverta::{
    module::{Module, Sens},
    stmt::{Case, Stmt},
};
use std::{fs, path::PathBuf};

const NAME: &str = "basic";

#[test]
fn test_basic() {
    let m = Module::new(NAME)
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
        )
        .always_comb(
            Stmt::begin()
                .case({
                    let a = Case::new("hoge");
                    a
                })
                .end(),
        );
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", NAME));
    fs::write(path, s).unwrap();
}
