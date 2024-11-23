use ruverta::{comb::Comb, module::Module};
use std::{fs, path::PathBuf};

const NAME: &str = "comb";

#[test]
fn test_comb() {
    let m = Module::new(NAME)
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 1)
        .input("in1", 1)
        .output("out0", 1)
        .output("out1", 1)
        .comb(
            Comb::new()
                .input("in0")
                .input("in1")
                .output("out0")
                .output("out1")
                .case("in0==0", vec!["0", "1"])
                .default(vec!["in0", "in1"]),
        );
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", NAME));
    fs::write(path, s).unwrap();
}
