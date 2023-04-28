pub mod basic;
pub mod list;
pub mod maths;
pub mod print;
pub mod throw;

use crate::run_state::RunState;

pub fn add_std_lib(run_state: &mut RunState) {
    basic::add_basic_lib(run_state);
    list::add_list_lib(run_state);
    maths::add_maths_lib(run_state);
    print::add_print_lib(run_state);
    throw::add_throw_lib(run_state);
}
