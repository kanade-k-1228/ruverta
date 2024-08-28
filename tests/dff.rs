use ruverta::{module::Module, stmt::Stmt};

#[test]
fn test_dff() {
    let m = Module::new("test_mod")
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
    println!("{}", m.verilog().join("\n"));
}