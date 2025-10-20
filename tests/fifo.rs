use ruverta::{ext::FIFO, mod_test, module::Module};

mod_test!(
    fifo,
    Module::new("fifo", "clk", "rstn").add(FIFO::new("rx", 8, 32))
);
