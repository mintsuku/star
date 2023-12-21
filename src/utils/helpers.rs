use flate2::read::GzDecoder;
use std::{fs::{File, self}, io::BufReader, env, path::Path};
use tar::Archive;
use lzma::reader::LzmaReader;
use dialoguer::{theme::ColorfulTheme, Select};
use super::enums::ArchiveType;





pub fn open_archive<P: AsRef<Path>>(filename: P, archive_type: ArchiveType) -> Archive<Box<dyn std::io::Read>> {
    let file = File::open(filename).expect("Error opening file");
    match archive_type {
        ArchiveType::Gz => {
            let buf_reader = BufReader::new(file);
            let gz_decoder = GzDecoder::new(buf_reader);
            Archive::new(Box::new(gz_decoder) as Box<dyn std::io::Read>)
        }
        ArchiveType::Xz => {
            let lzma_reader = LzmaReader::new_decompressor(file).unwrap();
            Archive::new(Box::new(lzma_reader) as Box<dyn std::io::Read>)
        }
    }
}

pub fn search_files(keyword: &str) -> Vec<String> {
    let current_dir = env::current_dir().expect("Failed to read current directory");
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                if file_name.contains(keyword) && (file_name.ends_with(".bz2") || file_name.ends_with(".xz") || file_name.ends_with(".gz")) {
                    results.push(path.to_string_lossy().into_owned());
                }
            }
        }
    }

    results
}

pub fn prompt_user_to_select(files: &[String]) -> std::io::Result<usize> {
    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a file ⭐")
        .items(&files)
        .interact()
    {
        Ok(selection) => Ok(selection),
        Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Dialoguer error: {}", err))),
    }
}

pub fn handle_filename(sub_matches: &clap::ArgMatches) -> String {
    sub_matches.get_one::<String>("FILE").unwrap_or_else(|| {
        eprintln!("File argument is missing");
        std::process::exit(1);
    }).clone()
}

pub fn extract_files<R: std::io::Read>(mut archive: Archive<R>, verbose: bool) -> Result<(), std::io::Error> {
    for file in archive.entries()? {
        let mut file = file?;

        if verbose {
            println!("Extracting file: {:?}", file.path()?);
        }

        let path = file.path()?.into_owned();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        if path.is_dir() {
            std::fs::create_dir_all(&path)?;
        } else {
            if let Err(e) = file.unpack(&path) {
                eprintln!("Error unpacking file {}: {}", path.display(), e);
                continue;
            }
        }
    }

    println!("Files extracted successfully to the current directory");

    Ok(())
}
