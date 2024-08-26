use crate::traits::Verilog;

#[derive(Debug)]
pub struct Module {
    name: String,
    param: Vec<Param>,
    port: Vec<Port>,
    stmt: Vec<Stmt>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            param: vec![],
            port: vec![],
            stmt: vec![],
        }
    }

    pub fn input(&mut self, name: &str, width: u32) {
        assert!(width > 0);
        self.port.push(Port {
            name: name.to_string(),
            direct: Direct::In,
            width: width,
            length: 1,
        })
    }

    pub fn output(&mut self, name: &str, width: u32) {
        assert!(width > 0);
        self.port.push(Port {
            name: name.to_string(),
            direct: Direct::Out,
            width: width,
            length: 1,
        })
    }

    pub fn inout(&mut self, name: &str, width: u32) {
        assert!(width > 0);
        self.port.push(Port {
            name: name.to_string(),
            direct: Direct::InOut,
            width: width,
            length: 1,
        })
    }

    pub fn param(&mut self, name: &str, default: Option<String>) {
        self.param.push(Param {
            name: name.to_string(),
            default: default,
        })
    }

    pub fn logic(&mut self, name: &str, width: u32, len: u32) {
        self.stmt.push(Stmt::Logic(Logic {
            name: name.to_string(),
            width: width,
            length: len,
        }))
    }
}

impl Verilog for Module {
    fn verilog(&self) -> String {
        let mut a: Vec<String> = Vec::new();
        a.push(format!("module {} #(", self.name));
        for param in &self.param {
            a.push(format!("  {},", param.verilog()))
        }
        a.push(format!(") ("));
        for port in &self.port {
            a.push(format!("  {},", port.verilog()))
        }
        a.push(format!(");"));
        for stmt in &self.stmt {
            a.push(stmt.verilog())
        }
        a.push(format!("endmodule;"));
        a.join("\n")
    }
}

#[test]
fn test_gen_module() {
    let mut m = Module {
        name: format!("test_mod"),
        port: vec![],
        param: vec![],
        stmt: vec![],
    };
    m.input("clk", 1);
    m.input("rstn", 1);
    m.input("in", 2);
    println!("{}", m.verilog());
}

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
fn test_gen_port() {
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

#[derive(Debug)]
enum Stmt {
    Logic(Logic),
    Param(Param),
    AlwaysFF(AlwaysFF),
    AlwaysComb(AlwaysComb),
}

impl Verilog for Stmt {
    fn verilog(&self) -> String {
        match self {
            Stmt::Logic(e) => e.verilog(),
            Stmt::Param(e) => e.verilog(),
            Stmt::AlwaysFF(e) => e.verilog(),
            Stmt::AlwaysComb(e) => e.verilog(),
        }
    }
}

#[derive(Debug)]
struct Logic {
    name: String,
    width: u32,
    length: u32,
}
impl Verilog for Logic {
    fn verilog(&self) -> String {
        format!("")
    }
}

#[derive(Debug)]
struct AlwaysFF {}
impl Verilog for AlwaysFF {
    fn verilog(&self) -> String {
        format!("")
    }
}

#[derive(Debug)]
struct AlwaysComb {}
impl Verilog for AlwaysComb {
    fn verilog(&self) -> String {
        format!("")
    }
}
