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
    pub fn begin() -> Block {
        Block::begin()
    }
    pub fn cond() -> Cond {
        Cond::new()
    }
}

impl Stmt {
    pub fn print(&self, indent: usize) -> String {
        let tab = "  ".repeat(indent);
        match self {
            Stmt::Block(Block { body }) => {
                let blk_str = body
                    .iter()
                    .map(|stmt| stmt.print(indent + 1))
                    .collect::<Vec<_>>()
                    .join("");
                format!("{tab}begin\n{blk_str}{tab}end\n")
            }
            Stmt::Assign(Assign { var, val }) => {
                format!("{tab}{} = {};\n", var, val)
            }
            Stmt::Cond(Cond { if_, else_ }) => {
                let mut result = String::new();
                for (i, (cond, body)) in if_.iter().enumerate() {
                    if i == 0 {
                        result.push_str(&format!("{tab}if ({})\n{}", cond, body.print(indent + 1)));
                    } else {
                        result.push_str(&format!(
                            "{tab}else if ({})\n{}",
                            cond,
                            body.print(indent + 1)
                        ));
                    }
                }
                if let Some(else_) = else_ {
                    result.push_str(&format!("{tab}else\n{}", else_.print(indent + 1)));
                }
                result
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
    pub fn end(self) -> Stmt {
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
