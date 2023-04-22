mod node;
mod parser;
mod tokeniser;

fn _print_tokens(source: &str) {
    let tokens = tokeniser::tokenise(source).unwrap();

    for token in tokens {
        println!("{:?}", token);
    }

    println!();
}

fn print_node(node: &node::Node, rec_level: usize) {
    match node.get_token() {
        Some(token) => {
            for _ in 0..rec_level {
                print!("\t");
            }
            println!("{:?}", token);
        }
        None => {}
    }

    for child in node.get_children() {
        print_node(&child.try_borrow().unwrap(), rec_level + 1);
    }
}

fn print_node_from_source(source: &str) {
    let tokens = tokeniser::tokenise(source).unwrap();
    match parser::parse(tokens) {
        Ok(parent_node) => {
            println!("{:?}", parent_node);
            print_node(&parent_node.try_borrow().unwrap(), 0);
            println!();
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }
}

fn main() {
    print_node_from_source("+ 1 2");
    print_node_from_source("(+ 1 2)");
    print_node_from_source("(+ (- 3 2) 2)");
}
