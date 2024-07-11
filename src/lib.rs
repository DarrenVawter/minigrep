use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    println!("\nSearching for '{}' in {} . . .\n", config.query, config.file_path);

    let contents = 
        if config.case_sensitive{
            fs::read_to_string(&config.file_path)?
        }else{
            fs::read_to_string(&config.file_path)?.to_lowercase()
        };

    for line in search(&config.query, &contents).unwrap_or_else(|| vec!["No matches found . . ."]) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Option<Vec<&'a str>>{

    let result: Vec<&str> = contents.lines().filter(|line| line.contains(query)).collect();

    if result.len() == 0 {
        return None;
    }else{
        Some(result)
    }

}

pub struct Config{

    query: String,
    file_path: String,
    case_sensitive: bool,

}

impl Config{

    pub fn build_config(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{

        args.next();

        let query = match args.next(){
            Some(query) => query,
            None => return Err("Expected query."),
        };
        
        let file_path = match args.next(){
            Some(file_path) => file_path,
            None => return Err("Expected file path."),
        };

        let case_sensitive = 
            match env::var("CASE_SENSITIVE"){
                Ok(data) => if data=="0" {false} else {true},
                Err(_) => true,
            };

        let query =
            if case_sensitive{
                query
            }
            else{
                query.to_lowercase()
            };

        Ok(Config{query, file_path, case_sensitive})

    }

}





#[cfg(test)]
mod tests{
    
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\\nRust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents).unwrap());
    }

    #[test]
    fn no_result(){
        let query = "abduct";
        let contents = "\\nRust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(None, search(query, contents));
    }


}