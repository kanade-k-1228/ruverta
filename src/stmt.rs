// ----------------------------------------------------------------------------

#[derive(Debug)]
pub enum Stmt {
    Empty,
    Block(Block),
    Assign(Assign),
    Case(Case),
    If(String, Box<Stmt>),
    ElIf(String, Box<Stmt>),
    Else(Box<Stmt>),
}

impl Stmt {
    pub fn empty() -> Self {
        Self::Empty
    }
    pub fn begin() -> Block {
        Block::begin()
    }
    pub fn assign(var: impl Into<String>, val: impl Into<String>) -> Self {
        Self::Assign(Assign::new(var, val))
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
            Stmt::Empty => vec![format!(";")],
            Stmt::Block(block) => block.verilog(assign_op),
            Stmt::Assign(assign) => vec![assign.verilog(assign_op)],
            Stmt::Case(case) => case.verilog(assign_op),
            Stmt::If(cond, stmt) => {
                let mut ret = vec![format!("if ({})", cond)];
                ret.extend(stmt.verilog(assign_op).iter().map(|s| format!("  {s}")));
                ret
            }
            Stmt::ElIf(cond, stmt) => {
                let mut ret = vec![format!("else if ({})", cond)];
                ret.extend(stmt.verilog(assign_op).iter().map(|s| format!("  {s}")));
                ret
            }
            Stmt::Else(stmt) => {
                let mut ret = vec![format!("else")];
                ret.extend(stmt.verilog(assign_op).iter().map(|s| format!("  {s}")));
                ret
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
    pub fn assign(mut self, var: &str, val: &str) -> Self {
        self.body.push(Stmt::Assign(Assign::new(var, val)));
        self
    }
    pub fn case(mut self, case: Case) -> Self {
        self.body.push(Stmt::Case(case));
        self
    }
    pub fn r#if(mut self, cond: &str, stmt: Stmt) -> Self {
        self.body.push(Stmt::If(cond.to_string(), Box::new(stmt)));
        self
    }
    pub fn elif(mut self, cond: &str, stmt: Stmt) -> Self {
        self.body.push(Stmt::ElIf(cond.to_string(), Box::new(stmt)));
        self
    }
    pub fn r#else(mut self, stmt: Stmt) -> Self {
        self.body.push(Stmt::Else(Box::new(stmt)));
        self
    }
    pub fn add(mut self, stmt: Stmt) -> Self {
        self.body.push(stmt);
        self
    }
    pub fn end(self) -> Stmt {
        Stmt::Block(self)
    }
}

impl Block {
    fn verilog(&self, assign_op: &str) -> Vec<String> {
        let mut blk_str = vec!["begin".to_string()];
        blk_str.extend(
            self.body
                .iter()
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
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Assign {
    var: String,
    val: String,
}

impl Assign {
    fn new(var: impl Into<String>, val: impl Into<String>) -> Self {
        Self {
            var: var.into(),
            val: val.into(),
        }
    }
}

impl Assign {
    fn verilog(&self, assign_op: &str) -> String {
        format!("{} {} {};", self.var, assign_op, self.val)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Case {
    var: String,
    case: Vec<(String, Stmt)>,
    default: Option<Box<Stmt>>,
}

impl Case {
    pub fn new(var: impl Into<String>) -> Self {
        Self {
            var: var.into(),
            case: vec![],
            default: None,
        }
    }
    pub fn case(mut self, cond: impl Into<String>, stmt: Stmt) -> Self {
        self.case.push((cond.into(), stmt));
        self
    }
    pub fn default(mut self, stmt: Stmt) -> Self {
        self.default = Some(Box::new(stmt));
        self
    }
}

impl Case {
    fn verilog(&self, assign_op: &str) -> Vec<String> {
        if self.case.len() == 0 && self.default.is_none() {
            println!("Case stmt must have case!");
            return vec![];
        }
        let mut ret = Vec::<String>::new();
        ret.push(format!("case ({})", self.var));

        for (cond, stmt) in &self.case {
            ret.push(format!("  {}: ", cond));
            ret.extend(stmt.verilog(assign_op).iter().map(|s| format!("  {s}")));
        }

        if let Some(stmt) = &self.default {
            ret.push(format!("  default: "));
            ret.extend(stmt.verilog(assign_op).iter().map(|s| format!("  {s}")));
        }

        ret.push(format!("endcase"));
        ret
    }
}
