use std::env;
use std::process;
use funcs::{Config, run};

mod funcs;



fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arg: {}" , err);
        process::exit(1);
    });
    
    println!("Search for: {}", config.query);
    println!("In file: {}", config.filename);
    
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}