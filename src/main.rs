//CLI entry point

use clap::Parser;

use binlifr::cli::{self.Cli};

fn main(){
    let cli = Cli::parse();
    if let Err(e) = cli::run(cli) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
