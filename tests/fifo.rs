use ruverta::{fifo::FIFO, mod_test, module::Module};

mod_test!(
    fifo,
    Module::new("fifo", "clk", "rstn").fifo(FIFO::new("rx", 8, 32))
);
