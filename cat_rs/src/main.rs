use clap::Parser;

fn main() {
    let cli = cat_rs::Args::parse();
    if let Err(e) = cat_rs::run(cli) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
