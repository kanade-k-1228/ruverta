use ruverta::{fifo::FIFO, mod_test, module::Module};

mod_test!(fifo, Module::new("fifo").fifo(FIFO::new("rx", 8, 32)));
