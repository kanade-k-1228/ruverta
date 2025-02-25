use ruverta::{comb::Comb, mod_test, module::Module};

mod_test!(
    comb,
    Module::new("comb", "clk", "rstn")
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
        )
);
