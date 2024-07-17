use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input file
    #[arg(default_value = "-")]
    input_file: String,

    /// Output file
    output_file: Option<String>,

    /// Precede each output line with the count of the number of times the line occurred in the input
    #[arg(short, long)]
    count: bool,
}

pub fn run(args: Args) -> MyResult<()> {
    let mut read_file =
        read_open(&args.input_file).map_err(|err| format!("{}: {}", args.input_file, err))?;

    let mut write_file = write_open(args.output_file.as_deref())
        .map_err(|err| format!("{}: {}", args.input_file, err))?;

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if args.count {
                write!(write_file, "{:>4} {}", count, text)?;
            } else {
                write!(write_file, "{}", text)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    loop {
        let bytes = read_file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(count, &previous)?;
    Ok(())
}

fn read_open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn write_open(filename: Option<&str>) -> MyResult<Box<dyn Write>> {
    match filename {
        Some(name) => Ok(Box::new(File::create(name)?)),
        None => Ok(Box::new(io::stdout())),
    }
}
