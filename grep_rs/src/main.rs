use clap::Parser;

fn main() {
    let args = grep_rs::Args::parse();
    if let Err(e) = grep_rs::run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
