use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u64,
}

fn main() {
    let cli = Cli::parse();

    println!("verbose: {:?}", cli.verbose);
}
