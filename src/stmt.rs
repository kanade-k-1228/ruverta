// ----------------------------------------------------------------------------

#[derive(Debug)]
pub enum Stmt {
    Block(Block),
    Assign(Assign),
    Cond(Cond),
}

impl Stmt {
    pub fn new() -> Self {
        Self::Block(Block { body: vec![] })
    }
    pub fn assign(var: &str, val: &str) -> Self {
        Self::Assign(Assign::new(var, val))
    }
    pub fn open() -> Block {
        Block::begin()
    }
    pub fn cond() -> Cond {
        Cond::new()
    }
}

impl Stmt {
    pub fn blocking(&self) -> Vec<String> {
        self.verilog("<=")
    }
    pub fn nonblocking(&self) -> Vec<String> {
        self.verilog("=")
    }

    fn verilog(&self, assign_op: &str) -> Vec<String> {
        match self {
            Stmt::Block(Block { body }) => {
                let mut blk_str = vec!["begin".to_string()];
                blk_str.extend(
                    body.iter()
                        .flat_map(|stmt| {
                            stmt.verilog(assign_op)
                                .iter()
                                .map(|s| format!("  {s}"))
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                );
                blk_str.push("end".to_string());
                blk_str
            }
            Stmt::Assign(Assign { var, val }) => {
                vec![format!("{var} {assign_op} {val};")]
            }
            Stmt::Cond(Cond { if_, else_ }) => {
                let mut a = if_
                    .iter()
                    .enumerate()
                    .flat_map(|(i, (cond, body))| {
                        if i == 0 {
                            let mut a = vec![format!("if ({})", cond)];
                            a.extend(body.verilog(assign_op).iter().map(|s| format!("  {s}")));
                            a
                        } else {
                            let mut a = vec![format!("else if ({})", cond)];
                            a.extend(body.verilog(assign_op).iter().map(|s| format!("  {s}")));
                            a
                        }
                    })
                    .collect::<Vec<_>>();
                if let Some(else_) = else_ {
                    a.extend(vec![format!("else")]);
                    a.extend(else_.verilog(assign_op).iter().map(|s| format!("  {s}")));
                }
                a
            }
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Block {
    body: Vec<Stmt>,
}

impl Block {
    fn begin() -> Self {
        Self { body: vec![] }
    }
    pub fn add(mut self, stmt: Stmt) -> Self {
        self.body.push(stmt);
        self
    }
    pub fn close(self) -> Stmt {
        Stmt::Block(self)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Assign {
    var: String,
    val: String,
}

impl Assign {
    fn new(var: &str, val: &str) -> Self {
        Self {
            var: var.to_string(),
            val: val.to_string(),
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Cond {
    if_: Vec<(String, Stmt)>,
    else_: Option<Box<Stmt>>,
}

impl Cond {
    fn new() -> Self {
        Cond {
            if_: vec![],
            else_: None,
        }
    }

    pub fn r#if(mut self, cond: &str, body: Stmt) -> Self {
        self.if_.push((cond.to_string(), body));
        self
    }

    pub fn r#else(mut self, body: Stmt) -> Stmt {
        self.else_ = Some(Box::new(body));
        Stmt::Cond(self)
    }
}
