// use crate::EntryType::*;
use clap::Parser;
use clap::ValueEnum;
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// target file path
    #[arg(default_value = ".")]
    path: Vec<String>,

    /// True if the last component of the pathname being examined matches pattern
    #[arg(short = 'n', long = "name")]
    names: Vec<Regex>,

    /// True if the file is of the specified type
    #[arg(short, long)]
    #[clap(value_enum)]
    r#type: Vec<EntryType>,
}

pub fn run(args: Args) -> MyResult<()> {
    // In the Clap process, args.path contains multiple paths.
    for path in args.path {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if (args.r#type.is_empty()
                        || args.r#type.iter().any(|entry_type| match entry_type {
                            EntryType::Dir => entry.file_type().is_dir(),
                            EntryType::File => entry.file_type().is_file(),
                            EntryType::Link => entry.file_type().is_symlink(),
                        }))
                        && (args.names.is_empty()
                            || args
                                .names
                                .iter()
                                .any(|re| re.is_match(&entry.file_name().to_string_lossy())))
                    {
                        println!("{}", entry.path().display())
                    }
                }
            }
        }
    }
    Ok(())
}
