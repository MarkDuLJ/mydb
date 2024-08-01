use mydb::sql::{token,tokenizer};

fn main() {
    let input = "CREATE DATABASE ABC;";
    let mut tokenizer = token::Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    for token in tokens {
        println!("{:?}", token);
    } 
}