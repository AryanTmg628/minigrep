use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Result<Config, &str> = Config::build(&args);

    match config {
        Ok(config) => {
            if let Err(err) = Config::run(config) {
                println!("--------------------{}-------------------------", err);
                process::exit(1);
            }
        }
        Err(_err) => {
            println!("--------------------Invalid Arguments-------------------------");
            println!("Expected, cargo run -- <qeurystring> <filepath>  ");
            println!("--------------------------------------------------------------");
            process::exit(1);
        }
    }
}
