use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

// fn parse_config(args: &[String]) -> Config{
//     let query = args[1].clone();
//     let filename = args[2].clone();
// }
    
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(
            Config {
                query,
                filename,
            }
        )
    }
}

impl Config {
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
}
