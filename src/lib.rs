//! A simple cat application
use std::error::Error;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// A simple cat application
pub struct Config {
    /// The files to concatenate
    #[arg(default_value = "-")]
    pub files: Vec<String>,

    /// Number lines
    #[arg(short, long)]
    pub number: bool,

    /// Number nonblank lines
    #[arg(short = 'b', long)]
    pub number_nonblank: bool,
}

/// A generic result type for catr
pub type CatResult<T> = Result<T, Box<dyn Error>>;

/// Runs the application with the specified configuration
///
/// # Examples
/// ```
/// fn main() {
///     // parse the command line arguments and run the application
///     if let Err(e) = get_args().and_then(catr::run) {
///         eprintln!("{}", e);
///         std::process::exit(1);
///     }
/// }
///
/// pub fn get_args() -> CatResult<Config> {
///     // parse the command line
///     let args = Config::parse();
///     Ok(args)
/// }
/// ```
pub fn run(config: Config) -> CatResult<()> {
    for filename in config.files {
        // open the file
        match open(&filename) {
            Err(err) =>
                eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut line_count = 0;

                for line in file.lines() {
                    let line = line?;

                    // don't increment the line count if the line is not empty
                    // and we have the number_nonblank option set
                    line_count = if config.number_nonblank && line.is_empty() {
                        line_count
                    } else {
                        line_count + 1
                    };

                    // determine whether to print the line number
                    let show_line = config.number ||
                        (config.number_nonblank && !line.is_empty());

                    // print the output
                    print_output(line_count, &line, show_line)
                }
            }
        };
    }

    Ok(())
}

/// Prints an output to the console with an optional line number
///
/// This method prints the output in cat format with 6 spaces to the left
/// followed by the line number (if show_line is set)
/// # Examples
/// ```
/// print_output(1, &line, true)
/// ```
fn print_output(line_count: i32, line: &str, show_line: bool) {
    if show_line {
        println!("{:6}\t{}", line_count, line);
    } else {
        println!("{}", line);
    }
}

/// Opens a file and returns a BufRead for it
///
/// This method opens a file and returns a BufferedReader
/// # Examples
/// ```
/// for filename in config.files {
///     // open the file
///     match open(&filename) {
///         Err(err) =>
///             eprintln!("Failed to open {}: {}", filename, err),
///         Ok(file) => {
///             for line in file.lines() {
///                 let line = line?;
///                 println!("{}", line);
///             }
///         }
///     }
/// }
/// ```
fn open(filename: &str) -> CatResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
