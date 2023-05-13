use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let content =
        // fs::read_to_string(config.filename).expect("Something went wrong reading the file");
        fs::read_to_string(config.filename)?;
    println!("With text:\n{}", content);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(arg: &[String]) -> Result<Config, &str> {
        if arg.len() < 3 {
            return Err("not enough arguments");
        }
        let query = arg[1].clone();
        let filename = arg[2].clone();
        Ok(Config { query, filename })
    }
}

// fn parse_config(arg: &[String]) -> Config {
//     let query = arg[1].clone();
//     let filename = arg[2].clone();
//     Config { query, filename }
// }
