use ruverta::{ext::DFF, mod_test, module::Module, stmt::Stmt};

mod_test!(blink, {
    let div: usize = 24;
    Module::new("blink", "clk", "rstn")
        .logic("cnt", div, 1)
        .add(DFF::sync(
            Stmt::assign("cnt", "0"),
            Stmt::assign("cnt", "cnt + 1"),
        ))
        .input("clk", 1)
        .input("rst", 1)
        .output("led", 1)
        .always_comb(Stmt::assign("led", format!("cnt[{}]", div - 1)))
});
