use std::env;

fn main() {
    let mode = env::args().nth(1).unwrap();
    let addr = env::args().nth(2).unwrap();
    println!("{}", mode);
    println!("{}", addr);
}
