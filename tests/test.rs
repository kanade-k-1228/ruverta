use ruverta::{axi_lite_slave::AXILiteSlave, module::Module, stmt::Stmt};

const NAME: &str = "uart";

#[test]
fn test_uart() {
    let uart = Module::new(NAME)
        .inout("clk", 1)
        .input("rstn", 1)
        .output("tx", 1)
        .input("rx", 1)
        .axi_lite_slave(
            "clk",
            "rstn",
            AXILiteSlave::new("csr", 32)
                .read_write("div", 32, 1)
                .read_write("tx_data", 8, 1)
                .read_only("rx_data", 8, 1),
        )
        .async_ff(
            "clk",
            "rstn",
            Stmt::begin()
                .add(Stmt::assign("buf", "0"))
                .add(Stmt::assign("cnt", "0"))
                .end(),
            Stmt::begin().add(Stmt::assign("buf", "a")).end(),
        );
    println!("{}", uart.verilog().join("\n"));
}
