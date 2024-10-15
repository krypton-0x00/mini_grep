use std::process;
use std::env;
use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config:Config = mini_grep::Config::new(&args).unwrap_or_else( |err| {
        println!("[-] Problem Parsing Arguments: {}",err);
        process::exit(1);
    });

    
    if let Err(e) = mini_grep::run(config){
        println!("[-] Error: {}",e);
        process::exit(1);
    }
     


}



