use crate::{
    module::{AlwaysFF, Module},
    stmt::Stmt,
};

pub struct Seq {
    clk: String,
    rst: String,
    init: Stmt,
    stmt: Stmt,
}

impl Seq {
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
    pub fn async_ff(mut self, seq: Seq) -> Self {
        self = self.always_ff({
            AlwaysFF::new().posedge(&seq.clk).negedge(&seq.rst).stmt(
                Stmt::cond()
                    .r#if(&format!("!{}", seq.rst), seq.init)
                    .r#else(seq.stmt),
            )
        });
        self
    }
    pub fn sync_ff(mut self, seq: Seq) -> Self {
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
