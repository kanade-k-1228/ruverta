use ruverta::{comb::Comb, module::Module};
use std::{fs, path::PathBuf};

#[test]
fn test_comb() {
    let name = "test_comb";
    let m = Module::new(name)
        .input("clk", 1)
        .input("rstn", 1)
        .input("hoge", 1)
        .comb(
            Comb::new()
                .input("in0")
                .input("in1")
                .output("out0")
                .output("out1")
                .case("in0==0", "out0=0", "out1=0")
                .default("0", "1"),
        );
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", name));
    fs::write(path, s).unwrap();
}
