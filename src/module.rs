// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Module {
    name: String,
    params: Vec<Param>,
    ports: Vec<Port>,
    blocks: Vec<Block>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            params: vec![],
            ports: vec![],
            blocks: vec![],
        }
    }

    pub fn input(mut self, name: &str, width: u32) -> Self {
        assert!(width > 0);
        self.ports.push(Port::input(name, width, 1));
        self
    }
    pub fn output(mut self, name: &str, width: u32) -> Self {
        assert!(width > 0);
        self.ports.push(Port::output(name, width, 1));
        self
    }
    pub fn inout(mut self, name: &str, width: u32) -> Self {
        assert!(width > 0);
        self.ports.push(Port::inout(name, width, 1));
        self
    }

    pub fn param(mut self, name: &str, default: Option<&str>) -> Self {
        self.params.push(Param::new(name, default));
        self
    }

    pub fn logic(mut self, name: &str, width: u32, len: u32) -> Self {
        self.blocks.push(Block::Logic(Logic {
            name: name.to_string(),
            width: width,
            length: len,
        }));
        self
    }

    pub fn instant(mut self, inst: Instant) -> Self {
        self.blocks.push(Block::Instant(inst));
        self
    }

    pub fn always_comb(mut self, a: AlwaysComb) -> Self {
        self.blocks.push(Block::AlwaysComb(a));
        self
    }

    pub fn always_ff(mut self, a: AlwaysFF) -> Self {
        self.blocks.push(Block::AlwaysFF(a));
        self
    }
}

impl Module {
    pub fn verilog(&self) -> Vec<String> {
        let mut code: Vec<String> = Vec::new();

        code.push(format!("module {} #(", self.name));

        for (i, param) in self.params.iter().enumerate() {
            if i < self.params.len() - 1 {
                code.push(format!("  {},", param.verilog()));
            } else {
                code.push(format!("  {}", param.verilog()));
            }
        }

        code.push(format!(") ("));

        for (i, port) in self.ports.iter().enumerate() {
            if i < self.ports.len() - 1 {
                code.push(format!("  {},", port.verilog()));
            } else {
                code.push(format!("  {}", port.verilog()));
            }
        }

        code.push(format!(");"));

        for stmt in &self.blocks {
            for line in stmt.verilog() {
                code.push(format!("  {line}"))
            }
        }

        code.push(format!("endmodule;"));

        code
    }
}

#[test]
fn test_module() {
    let m = Module::new("test_module")
        .param("BIT", Some("8"))
        .input("clk", 1)
        .input("rstn", 1)
        .input("in0", 8)
        .input("in1", 8)
        .output("out", 8)
        .always_comb(AlwaysComb::new().stmt("out = in0 + in1;"));
    println!("{}", m.verilog().join("\n"));
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct Port {
    name: String,
    direct: Direct,
    width: u32,
    length: u32,
}

impl Port {
    fn input(name: &str, width: u32, length: u32) -> Self {
        Self {
            name: name.to_string(),
            direct: Direct::In,
            width,
            length,
        }
    }
    fn output(name: &str, width: u32, length: u32) -> Self {
        Self {
            name: name.to_string(),
            direct: Direct::Out,
            width,
            length,
        }
    }
    fn inout(name: &str, width: u32, length: u32) -> Self {
        Self {
            name: name.to_string(),
            direct: Direct::InOut,
            width,
            length,
        }
    }
}

impl Port {
    fn verilog(&self) -> String {
        let width_str = if self.width == 1 {
            format!("       ")
        } else {
            format!("[{:>2}:0] ", self.width - 1)
        };
        let length_str = if self.length == 1 {
            format!("")
        } else {
            format!("[{:>2}:0]", self.width - 1)
        };
        format!(
            "{:<6} logic {}{}{}",
            self.direct.verilog(),
            width_str,
            self.name,
            length_str
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Direct {
    In,
    Out,
    InOut,
}

impl Direct {
    fn verilog(&self) -> String {
        match self {
            Direct::In => format!("input"),
            Direct::Out => format!("output"),
            Direct::InOut => format!("inout"),
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct Param {
    name: String,
    default: Option<String>,
}

impl Param {
    fn new(name: &str, default: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            default: default.map(|s| s.to_string()),
        }
    }
    fn verilog(&self) -> String {
        match &self.default {
            Some(default) => format!("parameter {} = {}", self.name, default),
            None => format!("parameter {}", self.name),
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
enum Block {
    Logic(Logic),
    Instant(Instant),
    AlwaysFF(AlwaysFF),
    AlwaysComb(AlwaysComb),
}

impl Block {
    fn verilog(&self) -> Vec<String> {
        match self {
            Block::Logic(e) => e.verilog(),
            Block::Instant(e) => e.verilog(),
            Block::AlwaysFF(e) => e.verilog(),
            Block::AlwaysComb(e) => e.verilog(),
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct Logic {
    name: String,
    width: u32,
    length: u32,
}

impl Logic {
    fn verilog(&self) -> Vec<String> {
        let width_str = if self.width == 1 {
            format!("       ")
        } else {
            format!("[{:>2}:0] ", self.width - 1)
        };
        let length_str = if self.length == 1 {
            format!("")
        } else {
            format!("[{:>2}:0]", self.width - 1)
        };
        vec![format!("logic {}{}{};", width_str, self.name, length_str)]
    }
}

#[test]
fn test_logic() {
    let logic = Logic {
        name: format!("test"),
        width: 8,
        length: 4,
    };
    println!("{}", logic.verilog().join("\n"));
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Instant {
    name: String,
    module: String,
    params: Vec<(String, String)>,
    ports: Vec<(String, String)>,
}

impl Instant {
    pub fn new(name: &str, module: &str) -> Self {
        Self {
            name: name.to_string(),
            module: module.to_string(),
            params: vec![],
            ports: vec![],
        }
    }
    pub fn param(mut self, param: &str, wire: &str) -> Self {
        self.params.push((param.to_string(), wire.to_string()));
        self
    }
    pub fn port(mut self, port: &str, wire: &str) -> Self {
        self.ports.push((port.to_string(), wire.to_string()));
        self
    }
}

impl Instant {
    pub fn verilog(&self) -> Vec<String> {
        let mut code: Vec<String> = Vec::new();

        code.push(format!("{} #(", self.module));

        for (i, (param, value)) in self.params.iter().enumerate() {
            let sep = if i < self.params.len() - 1 { "," } else { "" };
            code.push(format!("  .{}({}){}", param, value, sep));
        }

        code.push(format!(") {} (", self.name));

        for (i, (port, value)) in self.ports.iter().enumerate() {
            let sep = if i < self.ports.len() - 1 { "," } else { "" };
            code.push(format!("  .{}({}){}", port, value, sep));
        }

        code.push(format!(");"));

        code
    }
}

#[test]
fn test_instant() {
    let obj = Instant::new("i_hoge", "hoge")
        .port("clk", "clk")
        .port("rstn", "rstn");
    println!("{}", obj.verilog().join("\n"));
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AlwaysFF {
    edges: Vec<Edge>,
    stmt: Vec<String>,
}

impl AlwaysFF {
    pub fn new() -> Self {
        Self {
            edges: vec![],
            stmt: vec![],
        }
    }
    pub fn posedge(mut self, wire: &str) -> Self {
        self.edges.push(Edge::Posedge(wire.to_string()));
        self
    }
    pub fn negedge(mut self, wire: &str) -> Self {
        self.edges.push(Edge::Negedge(wire.to_string()));
        self
    }
    pub fn bothedge(mut self, wire: &str) -> Self {
        self.edges.push(Edge::Bothedge(wire.to_string()));
        self
    }
    pub fn stmt(mut self, stmt: &str) -> Self {
        self.stmt.push(stmt.to_string());
        self
    }
}

impl AlwaysFF {
    fn verilog(&self) -> Vec<String> {
        let edge_str = self
            .edges
            .iter()
            .map(|edge| edge.verilog())
            .collect::<Vec<_>>()
            .join(" or ");

        let mut code = Vec::<String>::new();
        code.push(format!("always_ff @({edge_str}) begin"));
        for s in &self.stmt {
            code.push(format!("  {s}"));
        }
        code.push(format!("end"));
        code
    }
}

#[derive(Debug)]
enum Edge {
    Posedge(String),
    Negedge(String),
    Bothedge(String),
}

impl Edge {
    fn verilog(&self) -> String {
        match self {
            Edge::Posedge(s) => format!("posedge {s}"),
            Edge::Negedge(s) => format!("negedge {s}"),
            Edge::Bothedge(s) => format!("{s}"),
        }
    }
}

#[test]
fn test_always_ff() {
    let a = AlwaysFF::new()
        .posedge("clk")
        .stmt("if (!rstn) begin")
        .stmt("  cnt <= 0;")
        .stmt("end else begin")
        .stmt("  cnt <= cnt + 1;")
        .stmt("end");
    println!("{}", a.verilog().join("\n"));
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AlwaysComb {
    stmt: Vec<String>,
}

impl AlwaysComb {
    pub fn new() -> Self {
        Self { stmt: vec![] }
    }
    pub fn stmt(mut self, stmt: &str) -> Self {
        self.stmt.push(stmt.to_string());
        self
    }
}

impl AlwaysComb {
    fn verilog(&self) -> Vec<String> {
        let mut code = Vec::<String>::new();
        code.push(format!("always_comb begin"));
        for s in &self.stmt {
            code.push(format!("  {s}"));
        }
        code.push(format!("end"));
        code
    }
}

#[test]
fn test_always_comb() {
    let a = AlwaysComb::new()
        .stmt("if (!rstn) begin")
        .stmt("  n_cnt = 0;")
        .stmt("end else begin")
        .stmt("  n_cnt = cnt + 1;")
        .stmt("end");
    println!("{}", a.verilog().join("\n"));
}

// ----------------------------------------------------------------------------

// #[derive(Debug)]
// enum Stmt {
//     Block {
//         body: Vec<Stmt>,
//     },
//     Assign {
//         var: String,
//         val: String,
//     },
//     IfEl {
//         ifs: Vec<(String, Stmt)>,
//         el: Option<Box<Stmt>>,
//     },
// }

// impl Stmt {
//     fn block() -> Self {
//         Stmt::Block { body: vec![] }
//     }

//     fn assign(mut self, var: &str, val: &str) -> Self {
//         if let Stmt::Block {
//             body: ref mut stmts,
//         } = self
//         {
//             stmts.push(Stmt::Assign {
//                 var: var.to_string(),
//                 val: val.to_string(),
//             });
//         }
//         self
//     }

//     fn if_stmt(mut self, cond: &str, body: Stmt) -> Self {
//         if let Stmt::Block {
//             body: ref mut stmts,
//         } = self
//         {
//             stmts.push(Stmt::IfEl {
//                 cond: cond.to_string(),
//                 body: Box::new(body),
//             });
//         }
//         self
//     }

//     fn elif(mut self, cond: &str, body: Stmt) -> Self {
//         if let Stmt::Block {
//             body: ref mut stmts,
//         } = self
//         {
//             stmts.push(Stmt::Elif {
//                 cond: cond.to_string(),
//                 body: Box::new(body),
//             });
//         }
//         self
//     }

//     fn else_stmt(mut self, body: Stmt) -> Self {
//         if let Stmt::Block {
//             body: ref mut stmts,
//         } = self
//         {
//             stmts.push(Stmt::Else {
//                 body: Box::new(body),
//             });
//         }
//         self
//     }
// }

// impl Stmt {
//     fn print(&self, indent: usize) -> String {
//         let tab = |n: usize| "  ".repeat(n);
//         match self {
//             Stmt::Block { body } => {
//                 format!(
//                     "begin\n{}\n{}end",
//                     body.iter()
//                         .map(|stmt| stmt.print(indent + 1))
//                         .collect::<Vec<_>>()
//                         .join("\n"),
//                     tab(indent)
//                 )
//             }
//             Stmt::Assign { var, val } => {
//                 format!("{}{} = {};", tab(indent), var, val)
//             }
//             Stmt::If { cond, body } => {
//                 format!("{}if ({}) {}", tab(indent), cond, body.print(indent))
//             }
//             Stmt::Elif { cond, body } => {
//                 format!("{}else if ({}) {}", tab(indent), cond, body.print(indent),)
//             }
//             Stmt::Else { body } => {
//                 format!("{}else {}", tab(indent), body.print(indent))
//             }
//         }
//     }
// }

// #[test]
// fn test_stmt() {
//     let stmt = Stmt::block()
//         .if_stmt("a == b", Stmt::block().assign("a", "1").assign("b", "1"))
//         .elif("c == d", Stmt::block().assign("c", "c - 1"))
//         .else_stmt(Stmt::block().assign("e", "2"));
//     println!("{:?}", stmt);
//     println!("{}", stmt.print(0));
// }
