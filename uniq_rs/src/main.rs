use clap::Parser;

fn main() {
    let args = uniq_rs::Args::parse();
    if let Err(e) = uniq_rs::run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
