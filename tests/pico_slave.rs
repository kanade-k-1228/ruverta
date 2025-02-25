use ruverta::{bus::RegList, mod_test, module::Module};

#[cfg(feature = "unstable")]
mod_test!(
    pico_slave,
    Module::new("pico_slave", "clk", "rstn")
        .input("clk", 1)
        .input("rstn", 1)
        .pico_slave(
            "mem",
            "clk",
            "rstn",
            RegList::new()
                .read_write("csr_rw", 8, 4)
                .read_only("csr_ro", 8, 1)
                .trigger("csr_tw")
                .allocate_greedy(32, 8),
        )
);
