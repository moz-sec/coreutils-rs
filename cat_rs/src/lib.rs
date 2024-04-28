use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{ArgGroup, Parser};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(ArgGroup::new("number").args(["number_lines", "number_nonblank_lines"])))]
pub struct Args {
    /// Input files
    #[arg(default_value = "-")]
    files: Vec<String>,

    ///  Number the non-blank output lines, starting at 1
    #[arg(short = 'b', long)]
    number_nonblank_lines: bool,

    /// Number the output lines, starting at 1
    #[arg(short = 'n', long)]
    number_lines: bool,
}

pub fn run(args: Args) -> MyResult<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if args.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{:6}\t{}", prev_num, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
