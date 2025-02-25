use ruverta::{
    mod_test,
    module::{Module, Sens},
    stmt::{Case, Stmt},
};

mod_test!(
    basic,
    Module::new("basic", "clk", "rstn")
        .param("BIT", Some("8"))
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 8)
        .input("in1", 8)
        .output("out", 8)
        .logic("tmp", 8, 1)
        .lparam("param", 0)
        .always_comb(Stmt::assign("tmp", "in0 + in1"))
        .always_ff(Sens::new().posedge("clk"), Stmt::assign("out", "tmp"))
        .always_comb(
            Stmt::begin()
                .case(
                    Case::new("hoge")
                        .case("0", Stmt::empty())
                        .case("1", Stmt::empty()),
                )
                .end(),
        )
);
