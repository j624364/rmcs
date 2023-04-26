pub mod basic;
pub mod maths;

use crate::run_state::RunState;

pub fn add_std_lib(run_state: &mut RunState) {
    basic::add_basic_lib(run_state);
    maths::add_maths_lib(run_state);
}