use crate::{
    module::{AlwaysFF, Module},
    stmt::Stmt,
};

pub struct Dff {
    clk: String,
    rst: String,
    init: Stmt,
    stmt: Stmt,
}

impl Dff {
    pub fn new(clk: &str, rst: &str, init: Stmt, stmt: Stmt) -> Self {
        Self {
            clk: clk.to_string(),
            rst: rst.to_string(),
            init,
            stmt,
        }
    }
}

impl Module {
    pub fn async_ff(mut self, seq: Dff) -> Self {
        self = self.always_ff({
            AlwaysFF::new().posedge(&seq.clk).negedge(&seq.rst).stmt(
                Stmt::cond()
                    .r#if(&format!("!{}", seq.rst), seq.init)
                    .r#else(seq.stmt),
            )
        });
        self
    }
    pub fn sync_ff(mut self, seq: Dff) -> Self {
        self = self.always_ff({
            AlwaysFF::new().posedge(&seq.clk).stmt(
                Stmt::cond()
                    .r#if(&format!("!{}", seq.rst), seq.init)
                    .r#else(seq.stmt),
            )
        });
        self
    }
}

#[test]
fn test_dff() {
    let m = Module::new("test_mod")
        .param("BIT", Some("8"))
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 8)
        .input("in1", 8)
        .output("out", 8)
        .sync_ff(Dff::new(
            "clk",
            "rstn",
            Stmt::open().add(Stmt::assign("out", "0")).close(),
            Stmt::open().add(Stmt::assign("out", "in0 + in1")).close(),
        ));
    println!("{}", m.verilog().join("\n"));
}
