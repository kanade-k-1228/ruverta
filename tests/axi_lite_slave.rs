use ruverta::{bus::RegList, mod_test, module::Module};

mod_test!(
    axi_lite_slave,
    Module::new("axi_lite_slave")
        .input("clk", 1)
        .input("rstn", 1)
        .axi_lite_slave(
            Some("cbus"),
            "clk",
            "rstn",
            RegList::new()
                .read_write("csr_rw", 8, 4)
                .read_only("csr_ro", 8, 1)
                .trigger("csr_tw")
                .allocate_greedy(32, 8),
        )
);
