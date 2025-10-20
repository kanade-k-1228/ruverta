use crate::{
    module::{Extension, Module},
    stmt::Stmt,
};

#[derive(Debug, Clone)]
pub struct Comb {
    inputs: Vec<String>,
    outputs: Vec<String>,
    cases: Vec<(String, Vec<String>)>,
    default: Vec<String>,
}

impl Comb {
    pub fn new() -> CombBuilder {
        CombBuilder {
            inputs: Vec::new(),
            outputs: Vec::new(),
            cases: Vec::new(),
        }
    }
}

pub struct CombBuilder {
    inputs: Vec<String>,
    outputs: Vec<String>,
    cases: Vec<(String, Vec<String>)>,
}

impl CombBuilder {
    pub fn input(mut self, name: impl ToString) -> Self {
        self.inputs.push(name.to_string());
        self
    }

    pub fn output(mut self, name: impl ToString) -> Self {
        self.outputs.push(name.to_string());
        self
    }

    pub fn case(mut self, cond: impl ToString, outs: Vec<impl ToString>) -> Self {
        assert!(self.outputs.len() == outs.len());
        self.cases.push((
            cond.to_string(),
            outs.into_iter().map(|s| s.to_string()).collect(),
        ));
        self
    }

    pub fn default(self, outs: Vec<impl ToString>) -> Comb {
        assert!(self.outputs.len() == outs.len());
        Comb {
            inputs: self.inputs,
            outputs: self.outputs,
            cases: self.cases,
            default: outs.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

// ----------------------------------------------------------------------------

impl Comb {
    pub fn build(self) {
        println!("Inputs: {:?}", self.inputs);
        println!("Outputs: {:?}", self.outputs);
        println!("Cases:");
        for (cond, outs) in self.cases {
            println!("  if {} => {:?}", cond, outs);
        }
        println!("Default: out0 = {:?}", self.default);
    }
}

impl Extension for Comb {
    fn add(self, mut module: Module) -> Module {
        module = module.always_comb({
            let mut stmt = Stmt::begin();
            for (cond, outs) in &self.cases {
                stmt = stmt.r#if(&cond, {
                    let mut stmt = Stmt::begin();
                    for (var, out) in outs.iter().zip(&self.outputs) {
                        stmt = stmt.assign(&out, var);
                    }
                    stmt.end()
                });
            }
            stmt = stmt.r#else({
                let mut stmt = Stmt::begin();
                for (var, out) in self.default.iter().zip(&self.outputs) {
                    stmt = stmt.assign(&out, var);
                }
                stmt.end()
            });
            stmt.end()
        });
        module
    }
}
