use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input string
    #[arg(required = true)]
    string: Vec<String>,

    /// Do not print the trailing newline character
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();

    let string = cli.string;
    let omit_newline = cli.omit_newline;

    print!(
        "{}{}",
        string.join(" "),
        if omit_newline { "" } else { "\n" }
    );
}
