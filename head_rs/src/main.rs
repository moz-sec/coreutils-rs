use clap::Parser;

fn main() {
    let args = head_rs::Args::parse();
    if let Err(e) = head_rs::run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
