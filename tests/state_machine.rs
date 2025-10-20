use ruverta::{ext::StateMachine, mod_test, module::Module};

mod_test!(state_machine, {
    const INIT: &str = "INIT";
    const RUNNING: &str = "RUNNING";
    let state_machine = StateMachine::new("state")
        .state(INIT)
        .jump("in0 == 1", RUNNING)
        .r#else(INIT)
        .state(RUNNING)
        .jump("in1 == 1", INIT)
        .r#else(RUNNING);
    Module::new("state_machine", "clk", "rstn")
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 1)
        .input("in1", 1)
        .add(state_machine)
});
