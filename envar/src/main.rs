use dotenv::dotenv;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    println!("args = {:?}", args);

    dotenv().ok();

    match env::var("SECRET_KEY") {
        Ok(val) => println!("SECRET_KEY={val}"),
        Err(e) => println!("Couldn't read SECRET_KEY, err = {:?}", e)
    };
}
