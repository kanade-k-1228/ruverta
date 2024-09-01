use crate::stmt::Stmt;

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
    pub fn input(mut self, name: &str, width: usize) -> Self {
        assert!(width > 0);
        self.ports.push(Port::input(name, width, 1));
        self
    }
    pub fn output(mut self, name: &str, width: usize) -> Self {
        assert!(width > 0);
        self.ports.push(Port::output(name, width, 1));
        self
    }
    pub fn inout(mut self, name: &str, width: usize) -> Self {
        assert!(width > 0);
        self.ports.push(Port::inout(name, width, 1));
        self
    }
    pub fn param(mut self, name: &str, default: Option<&str>) -> Self {
        self.params.push(Param::new(name, default));
        self
    }
    pub fn logic(mut self, name: &str, width: usize, len: usize) -> Self {
        self.blocks.push(Block::Logic(Logic::new(name, width, len)));
        self
    }
    pub fn instant(mut self, inst: Instant) -> Self {
        self.blocks.push(Block::Instant(inst));
        self
    }
    pub fn always_comb(mut self, stmt: Stmt) -> Self {
        self.blocks.push(Block::AlwaysComb(AlwaysComb::new(stmt)));
        self
    }
    pub fn always_ff(mut self, edges: Sens, stmt: Stmt) -> Self {
        self.blocks
            .push(Block::AlwaysFF(AlwaysFF::new(edges, stmt)));
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

        code.push(format!("endmodule"));

        code
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct Port {
    name: String,
    direct: Direct,
    width: usize,
    length: usize,
}

impl Port {
    fn input(name: &str, width: usize, length: usize) -> Self {
        Self {
            name: name.to_string(),
            direct: Direct::In,
            width,
            length,
        }
    }
    fn output(name: &str, width: usize, length: usize) -> Self {
        Self {
            name: name.to_string(),
            direct: Direct::Out,
            width,
            length,
        }
    }
    fn inout(name: &str, width: usize, length: usize) -> Self {
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
    width: usize,
    length: usize,
}

impl Logic {
    fn new(name: &str, width: usize, len: usize) -> Self {
        Self {
            name: name.to_string(),
            width: width,
            length: len,
        }
    }
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
    pub fn param(mut self, param: &str, val: &str) -> Self {
        self.params.push((param.to_string(), val.to_string()));
        self
    }
    pub fn port(mut self, port: &str, wire: &str) -> Self {
        self.ports.push((port.to_string(), wire.to_string()));
        self
    }
}

impl Instant {
    fn verilog(&self) -> Vec<String> {
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
        .port("rstn", "rstn")
        .param("a", "hoge");
    println!("{}", obj.verilog().join("\n"));
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AlwaysFF {
    sens: Sens,
    stmt: Stmt,
}

impl AlwaysFF {
    pub fn new(sens: Sens, stmt: Stmt) -> Self {
        Self { sens, stmt }
    }
}

impl AlwaysFF {
    fn verilog(&self) -> Vec<String> {
        let mut code = Vec::<String>::new();
        code.push(format!("always_ff @({})", self.sens.verilog()));
        code.extend(self.stmt.blocking().iter().map(|s| format!("  {s}")));
        code
    }
}

#[derive(Debug)]
pub struct Sens {
    edges: Vec<Edge>,
}

impl Sens {
    pub fn new() -> Self {
        Self { edges: vec![] }
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
}

impl Sens {
    fn verilog(&self) -> String {
        self.edges
            .iter()
            .map(|edge| edge.verilog())
            .collect::<Vec<_>>()
            .join(" or ")
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

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AlwaysComb {
    stmt: Stmt,
}

impl AlwaysComb {
    pub fn new(stmt: Stmt) -> Self {
        Self { stmt }
    }
}

impl AlwaysComb {
    fn verilog(&self) -> Vec<String> {
        let mut code = Vec::<String>::new();
        code.push(format!("always_comb"));
        code.extend(self.stmt.nonblocking().iter().map(|s| format!("  {s}")));
        code
    }
}
