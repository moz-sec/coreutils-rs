use clap::Parser;

fn main() {
    let args = find_rs::Args::parse();
    if let Err(e) = find_rs::run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
