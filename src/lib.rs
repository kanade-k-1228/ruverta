pub mod csr;
pub mod dff;
pub mod module;
pub mod stmt;

#[test]
fn test_csr() {
    let regmap = csr::RegMap::new("cbus", 32)
        .read_only("ctrl", 8, 2)
        .read_only("buf", 8, 1)
        .trigger("start");
    let module = module::Module::new("test_csr")
        .regmap(&regmap)
        .regio(&regmap);
    println!("{}", module.verilog().join("\n"))
}

#[test]
fn test_uart() {
    let uart = module::Module::new("uart")
        .inout("clk", 1)
        .input("rstn", 1)
        .output("tx", 1)
        .input("rx", 1)
        .regmap(
            &&csr::RegMap::new("csr", 32)
                .read_write("div", 32, 1)
                .read_write("tx_data", 8, 1)
                .read_only("rx_data", 8, 1),
        )
        .async_ff(dff::Dff::new(
            "clk",
            "rstn",
            stmt::Stmt::open()
                .add(stmt::Stmt::assign("buf", "0"))
                .add(stmt::Stmt::assign("cnt", "0"))
                .close(),
            stmt::Stmt::open()
                .add(stmt::Stmt::assign("buf", "a"))
                .close(),
        ));
    println!("{}", uart.verilog().join("\n"));
}

#[test]
fn test_stmt() {
    let stmt = stmt::Stmt::open()
        .add(
            stmt::Stmt::cond()
                .r#if(
                    "a == b",
                    stmt::Stmt::open()
                        .add(stmt::Stmt::assign("a", "1"))
                        .add(stmt::Stmt::assign("b", "1"))
                        .close(),
                )
                .r#if("c == d", stmt::Stmt::assign("c", "c - 1"))
                .r#else(stmt::Stmt::assign("e", "2")),
        )
        .close();
    println!("{}", stmt.verilog().join("\n"));
}
