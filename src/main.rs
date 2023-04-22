mod parser;

fn print_tokens(source: &str) {
    let tokens = parser::tokenise(source).unwrap();

    for token in tokens {
        println!("{:?}", token);
    }

    println!();
}

fn main() {
    print_tokens("(+ 1 (- 5 6))");
    print_tokens("(println \"Hello, World!\")");
}
