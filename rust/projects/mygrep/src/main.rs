use std::{env, fs, process, error::Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    println!("Contents of file:\n{contents}");

    Ok(())
}

#[derive(Debug)]
struct Config {
    path: String,
    query: String,
}

impl Config {
    fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let path = args[1].clone();
        let query = args[2].clone();

        Ok(Config{ path, query })
    }
}
