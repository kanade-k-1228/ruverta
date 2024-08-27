use crate::traits::Verilog;

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

    pub fn input(&mut self, name: &str, width: u32) {
        assert!(width > 0);
        self.ports.push(Port {
            name: name.to_string(),
            direct: Direct::In,
            width: width,
            length: 1,
        })
    }

    pub fn output(&mut self, name: &str, width: u32) {
        assert!(width > 0);
        self.ports.push(Port {
            name: name.to_string(),
            direct: Direct::Out,
            width: width,
            length: 1,
        })
    }

    pub fn inout(&mut self, name: &str, width: u32) {
        assert!(width > 0);
        self.ports.push(Port {
            name: name.to_string(),
            direct: Direct::InOut,
            width: width,
            length: 1,
        })
    }

    pub fn param(&mut self, name: &str, default: Option<String>) {
        self.params.push(Param {
            name: name.to_string(),
            default: default,
        })
    }

    pub fn logic(&mut self, name: &str, width: u32, len: u32) {
        self.blocks.push(Block::Logic(Logic {
            name: name.to_string(),
            width: width,
            length: len,
        }))
    }

    pub fn always_comb(&mut self, a: AlwaysComb) {
        self.blocks.push(Block::AlwaysComb(a));
    }

    pub fn always_ff(&mut self, a: AlwaysFF) {
        self.blocks.push(Block::AlwaysFF(a));
    }
}

impl Module {
    pub fn verilog(&self) -> Vec<String> {
        let mut code: Vec<String> = Vec::new();
        code.push(format!("module {} #(", self.name));
        for param in &self.params {
            code.push(format!("  {},", param.verilog()))
        }
        code.push(format!(") ("));
        for port in &self.ports {
            code.push(format!("  {},", port.verilog()))
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
    let mut m = Module {
        name: format!("test_mod"),
        ports: vec![],
        params: vec![],
        blocks: vec![],
    };
    m.input("clk", 1);
    m.input("rstn", 1);
    m.input("in", 2);
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

impl Verilog for Port {
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

#[test]
fn test_port() {
    let port = Port {
        name: format!("test_port"),
        direct: Direct::In,
        width: 2,
        length: 2,
    };
    println!("{}", port.verilog())
}

#[derive(Debug, Clone, Copy)]
enum Direct {
    In,
    Out,
    InOut,
}

impl Verilog for Direct {
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

impl Verilog for Param {
    fn verilog(&self) -> String {
        match &self.default {
            Some(default) => format!("param {} = {}", self.name, default),
            None => format!("param {}", self.name),
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
enum Block {
    Logic(Logic),
    AlwaysFF(AlwaysFF),
    AlwaysComb(AlwaysComb),
}

impl Block {
    fn verilog(&self) -> Vec<String> {
        match self {
            Block::Logic(e) => e.verilog(),
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
        vec![format!("logic {}{}{}", width_str, self.name, length_str)]
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
    pub fn posedge(&mut self, wire: &str) {
        self.edges.push(Edge::Posedge(wire.to_string()))
    }
    pub fn negedge(&mut self, wire: &str) {
        self.edges.push(Edge::Negedge(wire.to_string()))
    }
    pub fn bothedge(&mut self, wire: &str) {
        self.edges.push(Edge::Bothedge(wire.to_string()))
    }
    pub fn stmt(&mut self, stmt: &str) {
        self.stmt.push(stmt.to_string())
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

impl Verilog for Edge {
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
    let mut a = AlwaysFF::new();
    a.posedge("clk");
    a.stmt("if (!rstn) begin");
    a.stmt("  cnt <= 0;");
    a.stmt("end else begin");
    a.stmt("  cnt <= cnt + 1;");
    a.stmt("end");
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
    pub fn stmt(&mut self, stmt: &str) {
        self.stmt.push(stmt.to_string())
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
    let mut a = AlwaysComb::new();
    a.stmt("if (!rstn) begin");
    a.stmt("  n_cnt = 0;");
    a.stmt("end else begin");
    a.stmt("  n_cnt = cnt + 1;");
    a.stmt("end");
    println!("{}", a.verilog().join("\n"));
}

// ----------------------------------------------------------------------------

struct Stmt {}
