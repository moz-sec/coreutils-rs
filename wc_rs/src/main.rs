use clap::Parser;

fn main() {
    let args = wc_rs::Args::parse();
    if let Err(e) = wc_rs::run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
