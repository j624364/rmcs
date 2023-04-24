use std::fs;
use std::io;

pub type CmdOptionsErrorMessage = String;

#[derive(Debug, Clone, PartialEq)]
pub enum CodeSource {
    File(String),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CmdOptions {
    code_sources: Vec<CodeSource>,
    print_res: bool,
}

impl CmdOptions {
    pub fn parse(args: Vec<String>) -> Result<CmdOptions, CmdOptionsErrorMessage> {
        let mut code_sources = Vec::new();
        let mut print_res = false;

        enum CmdParsingMode {
            Normal,
            File,
            String,
        }

        let mut cmd_parsing_mode = CmdParsingMode::Normal;
        for arg in args {
            match cmd_parsing_mode {
                CmdParsingMode::Normal => match arg.as_str() {
                    "-i" | "--input" | "-f" | "--file" => {
                        cmd_parsing_mode = CmdParsingMode::File;
                    }
                    "-e" | "--eval" => {
                        cmd_parsing_mode = CmdParsingMode::String;
                    }
                    "-p" | "--print-res" => {
                        print_res = true;
                    }
                    _ => {
                        // assume file name
                        code_sources.push(CodeSource::File(arg));
                    }
                },
                CmdParsingMode::File => {
                    code_sources.push(CodeSource::File(arg));
                    cmd_parsing_mode = CmdParsingMode::Normal;
                }
                CmdParsingMode::String => {
                    code_sources.push(CodeSource::String(arg));
                    cmd_parsing_mode = CmdParsingMode::Normal;
                }
            }
        }

        Ok(Self {
            code_sources,
            print_res,
        })
    }

    pub fn should_print_res(&self) -> bool {
        self.print_res
    }

    pub fn get_code_sources(&self) -> Result<Vec<String>, io::Error> {
        let mut code_sources = Vec::with_capacity(self.code_sources.len());
        for code_source in &self.code_sources {
            code_sources.push(match code_source {
                CodeSource::File(path) => fs::read_to_string(path)?,
                CodeSource::String(string) => string.clone(),
            });
        }

        Ok(code_sources)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_input_sources_tests() {
        let input_path = "asdf".to_string();
        let code = "(+ 1 1)".to_string();

        // file
        assert_eq!(
            CmdOptions::parse(vec!["-i".to_string(), input_path.clone()]).unwrap(),
            CmdOptions {
                code_sources: vec![CodeSource::File(input_path.clone())],
                print_res: false
            }
        );

        // string
        assert_eq!(
            CmdOptions::parse(vec!["-e".to_string(), code.to_string()]).unwrap(),
            CmdOptions {
                code_sources: vec![CodeSource::String(code.clone())],
                print_res: false
            }
        );

        // file, string
        assert_eq!(
            CmdOptions::parse(vec![
                "-i".to_string(),
                input_path.clone(),
                "-e".to_string(),
                code.to_string()
            ])
            .unwrap(),
            CmdOptions {
                code_sources: vec![
                    CodeSource::File(input_path.clone()),
                    CodeSource::String(code.clone())
                ],
                print_res: false
            }
        );

        // string, file
        assert_eq!(
            CmdOptions::parse(vec![
                "-e".to_string(),
                code.to_string(),
                "-i".to_string(),
                input_path.clone(),
            ])
            .unwrap(),
            CmdOptions {
                code_sources: vec![
                    CodeSource::String(code.clone()),
                    CodeSource::File(input_path.clone()),
                ],
                print_res: false
            }
        );
    }

    #[test]
    fn flag_tests() {
        assert_eq!(
            CmdOptions::parse(vec!["-p".to_string()]).unwrap(),
            CmdOptions {
                code_sources: Vec::new(),
                print_res: true
            }
        );
    }
}
