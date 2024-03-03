use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
struct CliArgs {
    #[arg(short = 'c', long = "bytes")]
    count_bytes: bool,
    #[arg(short = 'l', long = "lines")]
    count_lines: bool,
    #[arg(short = 'w', long = "words")]
    count_words: bool,
    #[arg(short = 'm', long = "characters")]
    count_characters: bool,
    file_path: Option<PathBuf>,
}

fn count_bytes(content: &[u8]) -> Result<usize, std::io::Error> {
    return Ok(content.len());
}

fn count_lines(content: &mut String) -> Result<usize, std::io::Error> {
    let mut lines = 0;

    for symbol in content.chars() {
        if symbol == '\n' {
            lines += 1;
        }
    }
    return Ok(lines);
}

fn count_words(content: &mut String) -> Result<usize, std::io::Error> {
    let mut words = 0;
    let mut word_start = false;

    for symbol in content.chars() {
        if !symbol.is_ascii_whitespace() && !word_start {
            word_start = true;
        }
        if symbol.is_ascii_whitespace() && word_start {
            words += 1;
            word_start = false;
        }
    }
    return Ok(words);
}

fn count_characters(content: &mut String) -> Result<usize, std::io::Error> {
    let characters = content.chars().count();
    return Ok(characters);
}

fn main() -> Result<(), std::io::Error> {
    let args = CliArgs::parse();

    let mut input_src: Box<dyn io::Read> = match args.file_path.is_none() {
        true => Box::new(io::stdin()),
        false => Box::new(File::open(&args.file_path.as_ref().unwrap())?),
    };
    let mut content = &mut String::new();
    input_src.read_to_string(&mut content)?;

    let result = match true {
        _ if args.count_bytes => count_bytes(content.as_bytes())?.to_string(),
        _ if args.count_lines => count_lines(&mut content)?.to_string(),
        _ if args.count_words => count_words(&mut content)?.to_string(),
        _ if args.count_characters => count_characters(&mut content)?.to_string(),
        _ => {
            format!(
                "{} {} {} {}",
                count_bytes(content.as_bytes())?,
                count_lines(&mut content)?,
                count_words(&mut content)?,
                count_characters(&mut content)?,
            )
        }
    };

    let formatted_result = match args.file_path.is_none() {
        true => format!("{}\n", result),
        false => format!("{} {:?}\n", result, args.file_path.unwrap()),
    };

    io::stdout().write_all(formatted_result.as_bytes())?;
    Ok(())
}
