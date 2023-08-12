use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// specific type of Result that is either an Ok that always contains
// the unit type or some value that implements the std::error::Error trait

// Box indicates that the error will live inside a kind of pointer where
// the memory is dynamically allocated on the heap rather than the stack,
// and dyn indicates that the method calls on the std::error::Error trait
// are dynamically dispatched.

// the Ok part of TestResult will only ever hold the unit type, and the Err
// part can hold anything that implements the std::error::Error trait. These
// concepts are more thoroughly explained in the book Programming Rust.

type MyResult<T> = Result<T, Box<dyn Error>>;

// derive macro adds Debug trait so struct can be printed
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);

    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Professor Jiggly")
        .about("Rust version of cat")
        .arg(
            // file paths
            // arg is required and can be repeated
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Print with numbered lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Print with non blank lines numbered")
                .takes_value(false),
        )
        .get_matches();

    // there is a get_values() method but lossy version replaces invalid utf-8 codepoints (characters?)
    //  rather than panic
    let files = matches.values_of_lossy("files").unwrap(); // safe to call unwrap as we've specified a default value
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

// Box<T> type - pointer to heap-allocated value of type T
// use a Box here as we don't have a fixed, known size so can't store it on the stack
// can store it as a pointer as the pointer has a known size

// accept filename and return either filehandle (boxed value implementing BufRead trait) or an error
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
