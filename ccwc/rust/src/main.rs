use clap::Parser;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser)]
struct CliArgs {
    /// print the byte counts
    #[arg(short = 'c', long = "bytes")]
    count_bytes: bool,
    file_path: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = CliArgs::parse();
    if args.count_bytes {
        let file = File::open(&args.file_path)?;
        let metadata = file.metadata()?;
        let size = metadata.len();
        println!("{} {}", size, args.file_path.display());
    }
    Ok(())
}
