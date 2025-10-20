use ruverta::{ext::DFF, mod_test, module::Module, stmt::Stmt};

mod_test!(
    dff,
    Module::new("dff", "clk", "rstn")
        .param("BIT", Some("8"))
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 8)
        .input("in1", 8)
        .output("out", 8)
        .add(DFF::sync(
            Stmt::begin().assign("out", "0").end(),
            Stmt::begin().assign("out", "in0 + in1").end(),
        ))
);
