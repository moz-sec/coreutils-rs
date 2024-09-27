use anyhow::{anyhow, Result};
use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    mem,
};

use clap::Parser;
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Search pattern
    #[arg(required = true)]
    pattern: String,

    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Only a count of selected lines is written to standard output
    #[arg(short, long)]
    count: bool,

    /// Perform case insensitive matching
    #[arg(short, long)]
    ignore_case: bool,

    /// Selected lines are those not matching any of the specified patterns
    #[arg(short = 'v', long)]
    invert_match: bool,

    /// Recursively search subdirectories listed
    #[arg(short, long)]
    recursive: bool,
}

pub fn run(args: Args) -> MyResult<()> {
    let pattern = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .map_err(|_| format!("Invalid pattern \"{}\"", &args.pattern))?;

    let entries = find_files(&args.files, args.recursive);
    let num_files = entries.len();
    let print = |fname: &str, val: &str| {
        if num_files > 1 {
            print!("{fname}:{val}");
        } else {
            print!("{val}");
        }
    };

    for entry in entries {
        match entry {
            Ok(filename) => match open(&filename) {
                Ok(file) => match find_lines(file, &pattern, args.invert_match) {
                    Ok(matches) => {
                        if args.count {
                            print(&filename, &format!("{}\n", matches.len()));
                        } else {
                            for line in &matches {
                                print(&filename, line);
                            }
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                },
                Err(e) => eprintln!("{}: {}", filename, e),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}

fn find_files(paths: &[String], recursive: bool) -> Vec<Result<String>> {
    let mut results = vec![];

    for path in paths {
        match path.as_str() {
            "-" => results.push(Ok(path.to_string())),
            _ => match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if recursive {
                            for entry in WalkDir::new(path)
                                .into_iter()
                                .flatten()
                                .filter(|e| e.file_type().is_file())
                            {
                                results.push(Ok(entry.path().display().to_string()));
                            }
                        } else {
                            results.push(Err(anyhow!("{path} is a directory")));
                        }
                    } else if metadata.is_file() {
                        results.push(Ok(path.to_string()));
                    }
                }
                Err(e) => results.push(Err(anyhow!("{path}: {e}"))),
            },
        }
    }

    results
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn find_lines<T: BufRead>(mut file: T, pattern: &Regex, invert_match: bool) -> Result<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if pattern.is_match(&line) ^ invert_match {
            matches.push(mem::take(&mut line));
        }
        line.clear();
    }

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::find_lines;

    use super::find_files;
    use rand::{distributions::Alphanumeric, Rng};
    use regex::{Regex, RegexBuilder};

    #[test]
    fn test_find_files() {
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        let files = find_files(&["./tests/inputs/".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs/ is a directory")
        }

        let res = find_files(&["./tests/inputs/".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt"
            ]
        );

        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        let re1 = Regex::new("or").unwrap();

        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }
}
