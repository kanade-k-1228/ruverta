use crate::{module::Module, stmt::Stmt};

#[derive(Debug)]
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
    pub fn input(mut self, name: impl Into<String>) -> Self {
        self.inputs.push(name.into());
        self
    }

    pub fn output(mut self, name: impl Into<String>) -> Self {
        self.outputs.push(name.into());
        self
    }

    pub fn case(mut self, cond: impl Into<String>, outs: Vec<impl Into<String>>) -> Self {
        assert!(self.outputs.len() == outs.len());
        self.cases
            .push((cond.into(), outs.into_iter().map(Into::into).collect()));
        self
    }

    pub fn default(self, outs: Vec<impl Into<String>>) -> Comb {
        assert!(self.outputs.len() == outs.len());
        Comb {
            inputs: self.inputs,
            outputs: self.outputs,
            cases: self.cases,
            default: outs.into_iter().map(Into::into).collect(),
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

impl Module {
    pub fn comb(mut self, comb: Comb) -> Self {
        self = self.always_comb({
            let mut stmt = Stmt::begin();
            for (cond, outs) in &comb.cases {
                stmt = stmt.r#if(&cond, {
                    let mut stmt = Stmt::begin();
                    for (var, out) in outs.iter().zip(&comb.outputs) {
                        stmt = stmt.assign(&out, var);
                    }
                    stmt.end()
                });
            }
            stmt = stmt.r#else({
                let mut stmt = Stmt::begin();
                for (var, out) in comb.default.iter().zip(&comb.outputs) {
                    stmt = stmt.assign(&out, var);
                }
                stmt.end()
            });
            stmt.end()
        });
        self
    }
}
