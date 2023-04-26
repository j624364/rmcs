mod cmd_options;

use crate::cmd_options::CmdOptions;
use ::std::env;
use ::std::io;
use ryol::prelude::*;

pub fn handle_cmd_options(cmd_options: CmdOptions) -> Result<(), io::Error> {
    let code_sources = cmd_options.get_code_sources()?;

    let mut run_state = RunState::new();

    for source in code_sources {
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
    match CmdOptions::parse(env::args().collect()) {
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
