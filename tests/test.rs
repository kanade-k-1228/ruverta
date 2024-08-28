use ruverta::{dff::Dff, module::Module, regmap::RegMap, stmt::Stmt};

#[test]
fn test_csr() {
    let regmap = RegMap::new("cbus", 32)
        .read_only("ctrl", 8, 2)
        .read_only("buf", 8, 1)
        .trigger("start");
    let module = Module::new("test_csr").regmap(&regmap).regio(&regmap);
    println!("{}", module.verilog().join("\n"))
}

#[test]
fn test_uart() {
    let uart = Module::new("uart")
        .inout("clk", 1)
        .input("rstn", 1)
        .output("tx", 1)
        .input("rx", 1)
        .regmap(
            &RegMap::new("csr", 32)
                .read_write("div", 32, 1)
                .read_write("tx_data", 8, 1)
                .read_only("rx_data", 8, 1),
        )
        .async_ff(Dff::new(
            "clk",
            "rstn",
            Stmt::open()
                .add(Stmt::assign("buf", "0"))
                .add(Stmt::assign("cnt", "0"))
                .close(),
            Stmt::open().add(Stmt::assign("buf", "a")).close(),
        ));
    println!("{}", uart.verilog().join("\n"));
}
