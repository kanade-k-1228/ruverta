use std::collections::HashMap;

use crate::{module::Module, util::clog2};

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct FSM {
    state_var: String,
    clk: String,
    rst: String,
    states: HashMap<String, State>,
}

#[derive(Debug)]
struct State {
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
            states: HashMap::new(),
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

impl Module {
    pub fn sync_fsm(mut self, fsm: FSM) -> Self {
        let width = clog2(fsm.states.len()).unwrap_or(8);
        self = self.logic(&fsm.state_var, width, 1);
        self = Module::sync_ff(self, &fsm.clk, &fsm.rst, todo!(), todo!());
        self
    }
}

// ----------------------------------------------------------------------------

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
            trans: self.jumps,
            default: next.to_string(),
        };
        self.fsm.states.insert(self.name.clone(), state);
        self.fsm
    }
}
