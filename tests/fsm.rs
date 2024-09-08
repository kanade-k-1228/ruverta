use ruverta::{fsm::FSM, module::Module};
use std::{fs, path::PathBuf};

const NAME: &str = "fsm";

#[test]
fn test_fsm() {
    let m = Module::new(NAME).input("clk", 1).input("rstn", 1).sync_fsm(
        FSM::new("state", "clk", "rstn")
            .state("init")
            .jump("hoge == 1", "fuga")
            .r#else("init")
            .state("fuga")
            .jump("hoge == 0", "init")
            .r#else("fuga"),
    );
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", NAME));
    fs::write(path, s).unwrap();
}
