use ruverta::{bus::RegList, mod_test, module::Module, stmt::Stmt};

mod_test!(
    uart,
    Module::new("uart")
        .inout("clk", 1)
        .input("rstn", 1)
        .output("tx", 1)
        .input("rx", 1)
        .axi_lite_slave(
            Some("cbus"),
            "clk",
            "rstn",
            RegList::new()
                .read_write("div", 32, 1)
                .read_write("tx_data", 8, 1)
                .read_only("rx_data", 8, 1)
                .allocate_greedy(32, 8),
        )
        .async_ff(
            "clk",
            "rstn",
            Stmt::begin()
                .add(Stmt::assign("buffer", "0"))
                .add(Stmt::assign("cnt", "0"))
                .end(),
            Stmt::begin().add(Stmt::assign("buffer", "a")).end(),
        )
);
