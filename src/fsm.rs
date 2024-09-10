use crate::{
    module::Module,
    stmt::{Case, Stmt},
    util::clog2,
};

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct FSM {
    state_var: String,
    clk: String,
    rst: String,
    states: Vec<State>,
}

#[derive(Debug)]
struct State {
    name: String,
    trans: Vec<Trans>,
    default: String,
}

#[derive(Debug)]
struct Trans {
    cond: String,
    next: String,
}

impl FSM {
    pub fn new(state_var: &str, clk: &str, rst: &str) -> Self {
        FSM {
            state_var: state_var.to_string(),
            clk: clk.to_string(),
            rst: rst.to_string(),
            states: Vec::new(),
        }
    }

    pub fn state(self, name: &str) -> StateBuilder {
        StateBuilder {
            fsm: self,
            name: name.to_string(),
            jumps: vec![],
        }
    }
}

pub struct StateBuilder {
    fsm: FSM,
    name: String,
    jumps: Vec<Trans>,
}

impl StateBuilder {
    pub fn jump(mut self, cond: &str, next: &str) -> Self {
        self.jumps.push(Trans {
            cond: cond.to_string(),
            next: next.to_string(),
        });
        self
    }

    pub fn end(self) -> FSM {
        let a = self.name.clone();
        self.r#else(&a)
    }

    pub fn r#else(mut self, next: &str) -> FSM {
        let state = State {
            name: self.name,
            trans: self.jumps,
            default: next.to_string(),
        };
        self.fsm.states.push(state);
        self.fsm
    }
}

// ----------------------------------------------------------------------------

impl Module {
    pub fn sync_fsm(mut self, fsm: FSM) -> Self {
        println!("{:#?}", &fsm);
        let width = clog2(fsm.states.len()).unwrap_or(8);
        self = self.logic(&fsm.state_var, width, 1);
        for (i, state) in fsm.states.iter().enumerate() {
            self = self.lparam(&state.name, &format!("{i}"));
        }
        self = self.sync_ff(
            &fsm.clk,
            &fsm.rst,
            Stmt::assign(&fsm.state_var, &format!("0")),
            Stmt::begin()
                .case({
                    let mut cases = Case::new(&fsm.state_var);
                    for state in fsm.states {
                        cases = cases.case(&state.name, {
                            let mut stmt = Stmt::begin();
                            for trans in state.trans {
                                stmt = stmt
                                    .r#if(&trans.cond, Stmt::assign(&fsm.state_var, &trans.next));
                            }
                            stmt.end()
                        });
                    }
                    cases
                })
                .end(),
        );
        self
    }
}
