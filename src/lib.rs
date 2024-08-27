pub mod csr;
pub mod module;
pub mod seq;

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
        .always_ff(
            module::AlwaysFF::new()
                .posedge("clk")
                .stmt("if (!rstn) begin")
                .stmt("end else begin")
                .stmt("end"),
        )
        .regmap(&csr::RegMap::new("csr", 32).read_only("data", 8, 4));
    println!("{}", uart.verilog().join("\n"));
}
