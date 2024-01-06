pub mod cli;

use cli::cli;
use std::path::Path;



mod utils;
use utils::helpers::{extract_files, search_files, open_archive, handle_filename, prompt_user_to_select};
use utils::enums::ArchiveType;


fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("-gz", sub_matches)) => {
            let filename = handle_filename(sub_matches);
            let archive = open_archive(&filename, ArchiveType::Gz);
            let verbose = sub_matches.get_one::<bool>("verbose").expect("verbose");


            if let Err(e) = extract_files(archive, *verbose) {
                eprintln!("Error extracting .gz files: {}", e);
                std::process::exit(1);
            }
        }

        Some(("-xz", sub_matches)) => {
            println!("Matches: {:?}", sub_matches);
            let filename = handle_filename(sub_matches);
            let archive = open_archive(&filename, ArchiveType::Xz);
            let verbose = sub_matches.get_one::<bool>("verbose").expect("verbose");

            if let Err(e) = extract_files(archive, *verbose) {
                eprintln!("Error extracting .xz files: {}", e);
                std::process::exit(1);
            }
        }

        Some(("-bz2", sub_matches)) => {
            let filename = handle_filename(sub_matches);
            let archive = open_archive(&filename, ArchiveType::Bz2);
            let verbose = sub_matches.get_one::<bool>("verbose").expect("verbose");

            if let Err(e) = extract_files(archive, *verbose) {
                eprintln!("Error extracting .bz2 files: {}", e);
                std::process::exit(1);
            }
        }

        Some(("-search", sub_matches)) => {
            let keyword = sub_matches.get_one::<String>("KEYWORD").expect("No keyword provided");
            println!("Searching for files containing the keyword '{}'", keyword);
            let files = search_files(keyword);
            let verbose = sub_matches.get_one::<bool>("verbose").expect("verbose");


            if files.is_empty() {
                println!("No files found containing the keyword '{}'", keyword);
                return;
            }

            let selection = prompt_user_to_select(&files).expect("Failed to read user selection");

            println!("You selected: {}", files[selection]);

            let selected_file_path = Path::new(&files[selection]);
            let extension = selected_file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

            match extension {
                "gz" => {
                    let archive = open_archive(selected_file_path, ArchiveType::Gz);
                    if let Err(e) = extract_files(archive, *verbose) {
                        eprintln!("Error extracting .gz file: {}", e);
                    }
                }
                "xz" => {
                    let archive = open_archive(selected_file_path, ArchiveType::Xz);
                    if let Err(e) = extract_files(archive, *verbose) {
                        eprintln!("Error extracting .xz file: {}", e);
                    }
                }

                "bz2" => {
                    let archive = open_archive(selected_file_path, ArchiveType::Bz2);
                    if let Err(e) = extract_files(archive, *verbose) {
                        eprintln!("Error extracting .bz2 file: {}", e);
                    }
                }
                _ => {
                    println!("Unsupported file extension: {}", extension);
                }
            }
        }
        _ => unreachable!(),
    }
}

