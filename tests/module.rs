use ruverta::{
    module::{AlwaysComb, Module},
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
        .always_comb(AlwaysComb::new(Stmt::assign("out", "in0 + in1")));
    println!("{}", m.verilog().join("\n"));
}
