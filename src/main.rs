pub mod cli;

use clap::{Arg, Command};
use cli::cli;
use flate2::read::GzDecoder;
use std::{fs::File, io::BufReader};
use tar::{Archive, Entry};

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("-xzf", sub_matches)) => {
            let filename = sub_matches.get_one::<String>("FILE").unwrap_or_else(|| {
                eprintln!("File argument is missing");
                std::process::exit(1);
            });

            let file = match File::open(filename) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    std::process::exit(1);
                }
            };

            let buf_reader = BufReader::new(file);
            let gz_decoder = GzDecoder::new(buf_reader);
            let archive = Archive::new(gz_decoder);

            if let Err(e) = extract_files(archive) {
                eprintln!("Error extracting files: {}", e);
                std::process::exit(1);
            }
        }
        _ => unreachable!(),
    }
}

fn extract_files(mut archive: Archive<GzDecoder<BufReader<File>>>) -> Result<(), std::io::Error> {
    for file in archive.entries()? {
        let mut file = file?;

        let path = file.path()?.into_owned();

        if let Err(e) = file.unpack(&path) {
            eprintln!("Error unpacking file {}: {}", path.display(), e);
            continue;
        }

    }

    println!("Extraction over");


    Ok(())
}