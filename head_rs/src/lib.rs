use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

use clap::{ArgGroup, Parser};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(ArgGroup::new("range").args(["bytes", "lines"])))]
pub struct Args {
    /// Input files
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Print bytes of each of the specified files
    #[arg(short = 'c', long)]
    bytes: Option<usize>,

    /// Print count lines of each of the specified files
    #[arg(short = 'n', long, default_value_t = 10)]
    lines: usize,
}

pub fn run(args: Args) -> MyResult<()> {
    if args.lines == 0 {
        return Err(format!("illegal line count -- {}", args.lines).into());
    } else if args.bytes == Some(0) {
        return Err(format!("illegal byte count -- {}", args.bytes.unwrap()).into());
    }

    let num_files = args.files.len();

    for (file_num, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {} {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = args.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
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
