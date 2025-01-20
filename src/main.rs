use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            panic!("please, enter just 2 arguments, the word you want find and the file path");
        }
        let query = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text: \n {contents}");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("application error: {e}");
        process::exit(1);
    }
}
