use crate::{
    module::{Extension, Module},
    stmt::{Case, Stmt},
    util::clog2,
};

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct StateMachine {
    state_var: String,
    states: Vec<State>,
}

#[derive(Debug, Clone)]
struct State {
    name: String,
    trans: Vec<Trans>,
    default: String,
}

#[derive(Debug, Clone)]
struct Trans {
    cond: String,
    next: String,
}

impl StateMachine {
    pub fn new(state_var: impl ToString) -> Self {
        StateMachine {
            state_var: state_var.to_string(),
            states: Vec::new(),
        }
    }

    pub fn state(self, name: impl ToString) -> StateBuilder {
        StateBuilder {
            state_machine: self,
            name: name.to_string(),
            jumps: vec![],
        }
    }
}

pub struct StateBuilder {
    state_machine: StateMachine,
    name: String,
    jumps: Vec<Trans>,
}

impl StateBuilder {
    pub fn jump(mut self, cond: impl ToString, next: impl ToString) -> Self {
        self.jumps.push(Trans {
            cond: cond.to_string(),
            next: next.to_string(),
        });
        self
    }

    pub fn end(self) -> StateMachine {
        let a = self.name.clone();
        self.r#else(&a)
    }

    pub fn r#else(mut self, next: impl ToString) -> StateMachine {
        let state = State {
            name: self.name,
            trans: self.jumps,
            default: next.to_string(),
        };
        self.state_machine.states.push(state);
        self.state_machine
    }
}

// ----------------------------------------------------------------------------

impl Extension for StateMachine {
    fn add(self, mut module: Module) -> Module {
        println!("{:#?}", &self);
        let width = clog2(self.states.len()).unwrap_or(8);
        module = module.logic(&self.state_var, width, 1);
        for (i, state) in self.states.iter().enumerate() {
            module = module.lparam(&state.name, format!("{i}"));
        }

        use super::DFF;

        module = module.add(DFF::sync(
            Stmt::assign(&self.state_var, "0"),
            Stmt::begin()
                .case({
                    let mut cases = Case::new(&self.state_var);
                    for state in self.states {
                        cases = cases.case(&state.name, {
                            let mut stmt = Stmt::begin();
                            for trans in state.trans {
                                stmt = stmt
                                    .r#if(&trans.cond, Stmt::assign(&self.state_var, &trans.next));
                            }
                            stmt = stmt.r#else(Stmt::assign(&self.state_var, &state.default));
                            stmt.end()
                        });
                    }
                    cases
                })
                .end(),
        ));
        module
    }
}
