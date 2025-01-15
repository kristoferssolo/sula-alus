use std::io;

use clap::Parser;
use sula_alus::{process_input, Cli};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match process_input(&cli) {
        Ok(result) => {
            if let Some(s) = result {
                println!("{s}");
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}
