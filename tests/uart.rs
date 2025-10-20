use ruverta::{
    bus::{AXILiteSlave, RegList},
    ext::DFF,
    mod_test,
    module::Module,
    stmt::Stmt,
};

mod_test!(
    uart,
    Module::new("uart", "clk", "rstn")
        .inout("clk", 1)
        .input("rstn", 1)
        .output("tx", 1)
        .input("rx", 1)
        .add(AXILiteSlave::new(
            Some("cbus"),
            "clk",
            "rstn",
            RegList::new()
                .read_write("div", 32, 1)
                .read_write("tx_data", 8, 1)
                .read_only("rx_data", 8, 1)
                .allocate_greedy(32, 8),
        ))
        .add(DFF::r#async(
            Stmt::begin()
                .add(Stmt::assign("buffer", "0"))
                .add(Stmt::assign("cnt", "0"))
                .end(),
            Stmt::begin().add(Stmt::assign("buffer", "a")).end(),
        ))
);
