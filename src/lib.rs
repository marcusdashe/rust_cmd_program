use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { 
            query: query, 
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{

    contents.lines().filter(|line| line.contains(query)).collect()
    // let mut results = Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // }
    // results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
    
    // let mut results = Vec::new();

    // for line in contents.lines(){
    //     if line.to_lowercase().contains(&query){
    //         results.push(line);
    //     }
    // }
    // results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
            Rust: 
            safe, fast, productive. 
            Pick three.";
        
        assert_eq!(
            vec!["Safe, fast, productive."],
            search(query, contents)
        );
    }
}