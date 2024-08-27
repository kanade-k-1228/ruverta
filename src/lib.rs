pub mod csr;
pub mod module;
pub mod seq;
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
        .regmap(
            &csr::RegMap::new("csr", 32)
                .read_write("tx_buf", 8, 1)
                .read_write("rx_buf", 8, 1)
                .trigger("tx_start"),
        )
        .async_ff(seq::Seq::new(
            "clk",
            "rstn",
            stmt::Stmt::begin()
                .add(stmt::Stmt::assign("buf", "0"))
                .add(stmt::Stmt::assign("cnt", "0"))
                .end(),
            stmt::Stmt::begin()
                .add(stmt::Stmt::assign("buf", "a"))
                .end(),
        ));
    println!("{}", uart.verilog().join("\n"));
}

#[test]
fn test_stmt() {
    let stmt = stmt::Stmt::begin()
        .add(
            stmt::Stmt::cond()
                .r#if(
                    "a == b",
                    stmt::Stmt::begin()
                        .add(stmt::Stmt::assign("a", "1"))
                        .add(stmt::Stmt::assign("b", "1"))
                        .end(),
                )
                .r#if("c == d", stmt::Stmt::assign("c", "c - 1"))
                .r#else(stmt::Stmt::assign("e", "2")),
        )
        .end();
    println!("{}", stmt.print(0));
}
