use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{ArgGroup, Parser};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(ArgGroup::new("unit").args(["bytes", "chars"])))]
pub struct Args {
    /// Input files
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// The number of bytes in each input file
    #[arg(short, long)]
    bytes: bool,

    /// The number of lines in each input file
    #[arg(short, long)]
    lines: bool,

    /// The number of characters in each input file
    #[arg(short, long)]
    chars: bool,

    /// The number of words in each input file
    #[arg(short, long)]
    words: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_bytes: usize,
    num_lines: usize,
    num_chars: usize,
    num_words: usize,
}

pub fn run(mut args: Args) -> MyResult<()> {
    if [args.bytes, args.lines, args.chars, args.words]
        .iter()
        .all(|v| v == &false)
    {
        args.bytes = true;
        args.lines = true;
        args.words = true;
    }

    let mut total_num_bytes = 0;
    let mut total_num_lines = 0;
    let mut total_num_chars = 0;
    let mut total_num_words = 0;

    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, args.lines),
                        format_field(info.num_words, args.words),
                        format_field(info.num_bytes, args.bytes),
                        format_field(info.num_chars, args.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        },
                    );

                    total_num_lines += info.num_lines;
                    total_num_words += info.num_words;
                    total_num_bytes += info.num_bytes;
                    total_num_chars += info.num_chars;
                }
            }
        }
    }

    if args.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_num_lines, args.lines),
            format_field(total_num_words, args.words),
            format_field(total_num_bytes, args.bytes),
            format_field(total_num_chars, args.chars),
        );
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_bytes = 0;
    let mut num_lines = 0;
    let mut num_chars = 0;
    let mut num_words = 0;
    let mut buf = String::new();

    loop {
        let line_bytes = file.read_line(&mut buf)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_chars += buf.chars().count();
        num_words += buf.split_whitespace().count();
        buf.clear();
    }

    Ok(FileInfo {
        num_bytes,
        num_lines,
        num_chars,
        num_words,
    })
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::format_field;

    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_bytes: 48,
            num_lines: 1,
            num_chars: 48,
            num_words: 10,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
