use std::fs;
use std::error::Error;


pub fn run (config:Config)->Result<(),Box<dyn Error>> {
    let contents:String = fs::read_to_string(config.filename)?;

     
     for line in search(&config.query, &contents){
        println!("{}",line);
     }

     Ok(())
}
pub struct Config {
   pub query:String,
   pub filename: String
}

impl Config {
    pub fn new(args: &[String])->Result<Config,&str>{
        if args.len() < 3 {
            println!("[!] USAGE: mgrep <word> <filename>");
            return Err("[-] Not Enough Arguments.");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();
        Ok(Config {query,filename})
    }

}
 
 
 pub fn search<'a>(query: &str, contents: &'a str )->Vec<& 'a str>{

    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
 }