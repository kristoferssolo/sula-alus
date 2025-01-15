use std::{
    fs::{read_to_string, File},
    io::{self, Write},
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
}

pub fn encode(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn encode_string(s: &str) -> String {
    encode(s)
}

pub fn encode_file(path: impl Into<PathBuf>) -> io::Result<String> {
    let path = path.into();
    let content = read_to_string(path)?;
    Ok(encode(&content))
}

pub fn write_output(content: &str, path: &PathBuf) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn process_input(cli: &Cli) -> io::Result<Option<String>> {
    let result = if cli.file {
        encode_file(&cli.input)?
    } else {
        encode_string(&cli.input)
    };

    if let Some(output_path) = &cli.output {
        write_output(&result, &output_path)?;
        return Ok(None);
    }
    Ok(Some(result))
}
