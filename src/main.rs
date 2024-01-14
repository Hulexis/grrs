use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);
    let patt = args.pattern;

    for line in reader.lines() {
        parse_line(line?, patt.to_string())
    }

    println!("pattern: {:?}, path: {:?}", patt.to_string(), args.path);
    Ok(())
}

fn parse_line(line: String, pattern: String) {
    match &line {
        s if s.contains(&pattern) => println!("{}", s),
        _ => {}
    }
}
