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
    pub fn new(clk: impl ToString, rst: impl ToString, init: Stmt, stmt: Stmt) -> Self {
        Self {
            clk: clk.to_string(),
            rst: rst.to_string(),
            init,
            stmt,
        }
    }
}

impl Module {
    pub fn async_ff(
        mut self,
        clk: impl ToString,
        rst: impl ToString,
        init: Stmt,
        stmt: Stmt,
    ) -> Self {
        let dff = Dff::new(clk, rst, init, stmt);
        self = self.always_ff(
            Sens::new().posedge(&dff.clk).negedge(&dff.rst),
            Stmt::begin()
                .r#if(&format!("!{}", dff.rst), dff.init)
                .r#else(dff.stmt)
                .end(),
        );
        self
    }
    pub fn sync_ff(
        mut self,
        clk: impl ToString,
        rst: impl ToString,
        init: Stmt,
        stmt: Stmt,
    ) -> Self {
        let dff = Dff::new(clk, rst, init, stmt);
        self = self.always_ff(
            Sens::new().posedge(&dff.clk),
            Stmt::begin()
                .r#if(&format!("!{}", dff.rst), dff.init)
                .r#else(dff.stmt)
                .end(),
        );
        self
    }
}
