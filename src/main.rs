mod cmd_options;

use crate::cmd_options::{CmdOptions, CodeSource};
use ::std::env;
use ::std::io;
use ryol::prelude::*;
use std::fs;

pub fn handle_cmd_options(cmd_options: CmdOptions) -> Result<(), io::Error> {
    let mut run_state = RunState::new();

    for code_source in cmd_options.get_code_sources() {
        let source = match code_source {
            CodeSource::File(path) => fs::read_to_string(path)?,
            CodeSource::String(string) => string.clone(),
        };

        match run_state.eval(&source) {
            Ok(value) => {
                if cmd_options.should_print_res() {
                    println!("{}", value);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }

    Ok(())
}

fn main() {
    match CmdOptions::parse(env::args().skip(1).collect()) {
        Ok(cmd_options) => match handle_cmd_options(cmd_options) {
            Ok(()) => {}
            Err(error) => {
                println!("Error: \"{}\"!", error);
            }
        },
        Err(error_message) => {
            println!(
                "Error in parsing command line options: \"{}\"!",
                error_message
            );
        }
    }
}
