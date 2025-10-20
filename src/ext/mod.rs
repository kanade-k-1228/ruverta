//! Hardware component extensions
//!
//! This module contains various hardware component implementations that extend Module
//! functionality through the Extension trait.

pub mod comb;
pub mod dff;
pub mod fifo;
pub mod state_machine;
pub mod stream;

pub use comb::Comb;
pub use dff::DFF;
pub use fifo::FIFO;
pub use state_machine::StateMachine;
pub use stream::{Stream, StreamConfig};
