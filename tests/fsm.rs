use ruverta::{fsm::FSM, mod_test, module::Module};

mod_test!(fsm, {
    const INIT: &str = "INIT";
    const RUNNING: &str = "RUNNING";
    let fsm = FSM::new("state", "clk", "rstn")
        .state(INIT)
        .jump("in0 == 1", RUNNING)
        .r#else(INIT)
        .state(RUNNING)
        .jump("in1 == 1", INIT)
        .r#else(RUNNING);
    Module::new("fsm", "clk", "rstn")
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 1)
        .input("in1", 1)
        .sync_fsm(fsm)
});
