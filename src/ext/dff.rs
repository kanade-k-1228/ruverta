use crate::{
    module::{Extension, Module, Sens},
    stmt::Stmt,
};

/// DFF (D Flip-Flop) configuration
///
/// Represents a flip-flop configuration that can be added to a module.
#[derive(Debug, Clone)]
pub enum DFF {
    /// Synchronous reset flip-flop
    Sync { init: Stmt, stmt: Stmt },
    /// Asynchronous reset flip-flop
    Async { init: Stmt, stmt: Stmt },
}

impl DFF {
    /// Create a synchronous reset DFF configuration
    pub fn sync(init: Stmt, stmt: Stmt) -> Self {
        DFF::Sync { init, stmt }
    }

    /// Create an asynchronous reset DFF configuration
    pub fn r#async(init: Stmt, stmt: Stmt) -> Self {
        DFF::Async { init, stmt }
    }
}

impl Extension for DFF {
    fn add(self, mut module: Module) -> Module {
        let clk = module.clock.clone();
        let rst = module.reset.clone();

        match self {
            DFF::Async { init, stmt } => {
                module = module.always_ff(
                    Sens::new().posedge(clk).negedge(rst.clone()),
                    Stmt::begin()
                        .r#if(&format!("!{}", rst), init)
                        .r#else(stmt)
                        .end(),
                );
            }
            DFF::Sync { init, stmt } => {
                module = module.always_ff(
                    Sens::new().posedge(clk),
                    Stmt::begin()
                        .r#if(&format!("!{}", rst), init)
                        .r#else(stmt)
                        .end(),
                );
            }
        }

        module
    }
}
