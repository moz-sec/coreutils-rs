use clap::Parser;

fn main() {
    let args = cut_rs::Args::parse();
    if let Err(e) = cut_rs::run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
