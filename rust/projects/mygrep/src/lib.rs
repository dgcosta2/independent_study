use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub query: String,
}

impl Config {
    pub fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let path = args[1].clone();
        let query = args[2].clone();

        Ok(Config{ path, query })
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    println!("Contents of file:\n{contents}");

    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
}
