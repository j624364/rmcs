mod tokeniser;

fn _print_tokens(source: &str) {
    let tokens = tokeniser::tokenise(source).unwrap();

    for token in tokens {
        println!("{:?}", token);
    }

    println!();
}

fn main() {}
