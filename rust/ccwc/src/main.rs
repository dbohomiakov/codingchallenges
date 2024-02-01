use clap::Parser;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Parser)]
struct CliArgs {
    #[arg(short = 'c')]
    count_bytes: bool,
    file_path: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = CliArgs::parse();
    let mut file = File::open(args.file_path)?;
    let size = file.seek(SeekFrom::End(0))?;
    println!("{} {}", size, args.file_path);
    Ok(())
}
