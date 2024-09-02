use crate::module::Module;

#[derive(Debug)]
pub struct Comb {
    inputs: Vec<String>,
    outputs: Vec<String>,
    cases: Vec<Case>,
    default: Option<(String, String)>,
}

#[derive(Debug)]
struct Case {
    condition: String,
    output0: String,
    output1: String,
}

impl Comb {
    pub fn new() -> Self {
        Comb {
            inputs: Vec::new(),
            outputs: Vec::new(),
            cases: Vec::new(),
            default: None,
        }
    }

    pub fn input(mut self, name: &str) -> Self {
        self.inputs.push(name.to_string());
        self
    }

    pub fn output(mut self, name: &str) -> Self {
        self.outputs.push(name.to_string());
        self
    }

    pub fn case(mut self, condition: &str, output0: &str, output1: &str) -> Self {
        self.cases.push(Case {
            condition: condition.to_string(),
            output0: output0.to_string(),
            output1: output1.to_string(),
        });
        self
    }

    pub fn default(mut self, output0: &str, output1: &str) -> Self {
        self.default = Some((output0.to_string(), output1.to_string()));
        self
    }

    pub fn build(self) {
        println!("Inputs: {:?}", self.inputs);
        println!("Outputs: {:?}", self.outputs);
        println!("Cases:");
        for case in self.cases {
            println!(
                "  if {} => out0 = {}, out1 = {}",
                case.condition, case.output0, case.output1
            );
        }
        if let Some((out0, out1)) = self.default {
            println!("Default: out0 = {}, out1 = {}", out0, out1);
        }
    }
}

impl Module {
    pub fn comb(mut self, comb: Comb) -> Self {
        todo!();
        self
    }
}
