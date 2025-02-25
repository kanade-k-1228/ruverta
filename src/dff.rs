use crate::{
    module::{Module, Sens},
    stmt::Stmt,
};

impl Module {
    pub fn async_ff(mut self, init: Stmt, stmt: Stmt) -> Self {
        let clk = self.clock.clone();
        let rst = self.reset.clone();
        self = self.always_ff(
            Sens::new().posedge(clk).negedge(rst.clone()),
            Stmt::begin()
                .r#if(&format!("!{}", rst), init)
                .r#else(stmt)
                .end(),
        );
        self
    }
    pub fn sync_ff(mut self, init: Stmt, stmt: Stmt) -> Self {
        let clk = self.clock.clone();
        let rst = self.reset.clone();
        self = self.always_ff(
            Sens::new().posedge(clk),
            Stmt::begin()
                .r#if(&format!("!{}", rst), init)
                .r#else(stmt)
                .end(),
        );
        self
    }
}
