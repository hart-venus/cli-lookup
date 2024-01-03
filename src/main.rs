use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: String = args[1..].join(" ");

    println!("Hello, world! {}", args);
}
