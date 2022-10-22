use clap::Parser;
use catr::{Config, CatResult};

/// Parses commandline arguments and concatenates passed files.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// catr -n test/input/*
/// ```
fn main() {
    // parse the command line arguments and run the application
    if let Err(e) = get_args().and_then(catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn get_args() -> CatResult<Config> {
    // parse the command line
    let args = Config::parse();
    Ok(args)
}