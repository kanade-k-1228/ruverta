use ruverta::{module::Module, regmap::RegMap};

#[test]
fn test_regmap() {
    let module = Module::new("test_csr").regmap(
        RegMap::new("cbus", 32)
            .read_only("ctrl", 8, 2)
            .read_only("buf", 8, 1)
            .trigger("start"),
    );
    println!("{}", module.verilog().join("\n"))
}
