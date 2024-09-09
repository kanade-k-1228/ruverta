use ruverta::{fsm::FSM, module::Module};
use std::{fs, path::PathBuf};

const NAME: &str = "fsm";

const INIT: &str = "INIT";
const RUNNING: &str = "RUNNING";

#[test]
fn test_fsm() {
    let fsm = FSM::new("state", "clk", "rstn")
        .state(INIT)
        .jump("in0 == 1", RUNNING)
        .r#else(INIT)
        .state(RUNNING)
        .jump("in1 == 1", INIT)
        .r#else(RUNNING);
    let m = Module::new(NAME)
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 1)
        .input("in1", 1)
        .sync_fsm(fsm);
    let s = m.verilog().join("\n");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("tests/verilog/{}.sv", NAME));
    fs::write(path, s).unwrap();
}
