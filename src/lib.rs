use std::{
    fs::{read_to_string, write},
    io::{self},
    path::PathBuf,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The input text or file path to hash
    pub input: String,
    /// Specify if the input is a file
    #[arg(short, long)]
    pub file: bool,
    /// Output file path (optional)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    /// Reverse by line instead of the whole content
    #[arg(short, long)]
    pub line_by_line: bool,
}

pub fn encode(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn encode_string(s: &str) -> String {
    encode(s)
}

pub fn encode_file(path: impl Into<PathBuf>, line_by_line: bool) -> io::Result<String> {
    let path = path.into();
    let content = read_to_string(path)?;
    if line_by_line {
        return Ok(content
            .lines()
            .map(|line| encode(line))
            .collect::<Vec<String>>()
            .join("\n"));
    }
    Ok(encode(&content))
}

pub fn process_input(cli: &Cli) -> io::Result<Option<String>> {
    let result = if cli.file {
        encode_file(&cli.input, cli.line_by_line)?
    } else {
        encode_string(&cli.input)
    };

    if let Some(output_path) = &cli.output {
        write(output_path, result)?;
        return Ok(None);
    }
    Ok(Some(result))
}
