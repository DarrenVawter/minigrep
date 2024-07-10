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

    let mut result = vec![];

    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }

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

    pub fn build_config(args: Vec<String>) -> Result<Config, &'static str>{

        if args.len()!=3{
            return Err("Expected 2 input arguments.");
        } 

        let file_path = args[2].clone();
        let case_sensitive = 
            match env::var("CASE_SENSITIVE"){
                Ok(data) => if data=="0" {false} else {true},
                Err(_) => true,
            };
        let query =
            if case_sensitive{
                args[1].clone()
            }
            else{
                args[1].clone().to_lowercase()
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