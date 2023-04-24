mod cmd_options;
mod error;
mod function;
mod node;
mod parser;
mod run_state;
mod std;
mod tokeniser;
mod value;
mod variable;

use crate::cmd_options::CmdOptions;
use crate::run_state::RunState;
use ::std::env;
use ::std::io;

pub fn handle_cmd_options(cmd_options: CmdOptions) -> Result<(), io::Error> {
    let code_sources = cmd_options.get_code_sources()?;

    let mut run_state = RunState::new();

    for source in code_sources {
        match tokeniser::tokenise(source.as_str()) {
            Ok(tokens) => {
                match parser::parse(tokens) {
                    Ok(parent_node) => {
                        match parent_node.evaluate(&mut run_state) {
                            Ok(value) => {
                                if cmd_options.should_print_res() {
                                    println!("{}", value);
                                }
                            }
                            Err(error) => {
                                // todo: use Display
                                println!("Runtime error: \"{:?}\"", error);
                            }
                        }
                    }
                    Err(parser_error) => {
                        // todo: use Display for token
                        println!(
                            "Parser error: \"{}\" at token: {:?}",
                            parser_error.get_message(),
                            parser_error.get_token()
                        );
                    }
                }
            }
            Err(tokeniser_error) => {
                println!(
                    "Tokeniser error: \"{}\" at {}:{}",
                    tokeniser_error.get_message(),
                    tokeniser_error.get_line_no(),
                    tokeniser_error.get_col_no()
                );
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
