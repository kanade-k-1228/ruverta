use ruverta::{
    module::{Module, Sens},
    stmt::Stmt,
};

#[test]
fn test_module() {
    let m = Module::new("test_module")
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
    println!("{}", m.verilog().join("\n"));
}
