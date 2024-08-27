use crate::module::{AlwaysFF, Module};

pub struct Seq {
    clk: String,
    rst: String,
    init: Vec<String>,
    stmt: Vec<String>,
}

impl Seq {
    pub fn new(clk: &str, rst: &str) -> Self {
        Self {
            clk: clk.to_string(),
            rst: rst.to_string(),
            init: vec![],
            stmt: vec![],
        }
    }
    pub fn init(&mut self, wire: &str, val: &str) {
        self.init.push(format!("{wire} = {val};"));
    }
    pub fn stmt(&mut self, s: &str) {
        self.stmt.push(s.to_string());
    }
}

impl Module {
    pub fn seq(mut self, seq: &Seq) -> Self {
        self = self.always_ff({
            let mut a = AlwaysFF::new()
                .posedge(&seq.clk)
                .stmt(&format!("if (!{}) begin", seq.rst));
            for s in &seq.init {
                a = a.stmt(&format!("  {s}"));
            }
            a = a.stmt("end else begin");
            for s in &seq.stmt {
                a = a.stmt(&format!("  {s}"));
            }
            a.stmt("end")
        });
        self
    }
}
