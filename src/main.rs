use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let test = std::env::var("TEST_KEY1").expect("Couldnt find shit");
    println!("{}", test);
}