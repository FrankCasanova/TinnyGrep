use std::env;
use std::process;

use command_line_software::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsin arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}
