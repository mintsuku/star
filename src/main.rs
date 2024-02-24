pub mod cli;

use cli::cli;
use std::path::Path;




mod utils;
use utils::helpers::{create_archive_from_files, extract_files, handle_filename, list_archive_contents, open_archive, prompt_user_to_select, search_files};
use utils::enums::ArchiveType;
use std::fs;


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


        Some(("-c", sub_matches)) => {
            let filename = handle_filename(sub_matches);
            let verbose = sub_matches.get_one::<bool>("verbose").expect("verbose");
            let file_arg = sub_matches.get_one::<String>("FILE").expect("No file provided");
            let file_type = sub_matches.get_one::<String>("TYPE").expect("No file type provided");
            let mut archive_type: Option<ArchiveType> = None;
            match file_type.as_str() {
                "gz" => {
                    archive_type = Some(ArchiveType::Gz);
                    println!("Creating .tar.gz archive");
                }
                "xz" => {
                    archive_type = Some(ArchiveType::Xz);
                }
                "bz2" => {
                    archive_type = Some(ArchiveType::Bz2);
                }
                _ => {
                    eprintln!("Unsupported file type: {}", file_type);
                    std::process::exit(1);
                }
            }
            let file_path = Path::new(file_arg);
            if file_path.exists() {
                if file_path.is_dir() { // Changed this line
                    // Directory is provided
                    let dir = fs::read_dir(file_path).expect("Failed to read directory");
                    let files = dir.filter_map(Result::ok).map(|entry| entry.path()).collect::<Vec<_>>();
                    println!("Creating archive from files: {:?}", files);
                    create_archive_from_files(files, &filename, *verbose, *archive_type.as_ref().unwrap()).expect("Failed to create archive");
                }
            } else {
                eprintln!("File or directory does not exist");
                std::process::exit(1);
            }
        }

        Some(("-l", sub_matches)) => {
            let filename = handle_filename(sub_matches);
            if let Err(e) = list_archive_contents(filename) {
                eprintln!("Error listing archive contents: {}", e);
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

