pub mod basic;
pub mod list;
pub mod maths;
pub mod print;
pub mod string;
pub mod structure;
pub mod throw;

use crate::prelude::*;

pub fn add_std_lib(run_state: &mut RunState) -> Result<(), Error> {
    basic::add_basic_lib(run_state)?;
    list::add_list_lib(run_state)?;
    maths::add_maths_lib(run_state)?;
    print::add_print_lib(run_state)?;
    throw::add_throw_lib(run_state)?;
    string::add_string_lib(run_state)?;
    structure::add_structure_lib(run_state)?;

    Ok(())
}
