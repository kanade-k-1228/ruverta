use crate::{
    module::{Module, Sens},
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
    pub fn async_ff(mut self, clk: &str, rst: &str, init: Stmt, stmt: Stmt) -> Self {
        let dff = Dff::new(clk, rst, init, stmt);
        self = self.always_ff(
            Sens::new().posedge(&dff.clk).negedge(&dff.rst),
            Stmt::cond()
                .r#if(&format!("!{}", dff.rst), dff.init)
                .r#else(dff.stmt),
        );
        self
    }
    pub fn sync_ff(mut self, clk: &str, rst: &str, init: Stmt, stmt: Stmt) -> Self {
        let dff = Dff::new(clk, rst, init, stmt);
        self = self.always_ff(
            Sens::new().posedge(&dff.clk),
            Stmt::cond()
                .r#if(&format!("!{}", dff.rst), dff.init)
                .r#else(dff.stmt),
        );
        self
    }
}
