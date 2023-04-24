pub mod maths;

use crate::run_state::RunState;

pub fn add_std_lib(run_state: &mut RunState) {
    maths::add_maths_lib(run_state);
}
