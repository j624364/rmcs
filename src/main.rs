mod parser;

fn _print_tokens(source: &str) {
    let tokens = parser::tokenise(source).unwrap();

    for token in tokens {
        println!("{:?}", token);
    }

    println!();
}

fn main() {}
