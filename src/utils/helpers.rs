use super::enums::ArchiveType;
use bzip2::{read::BzDecoder, write::BzEncoder};
use dialoguer::{theme::ColorfulTheme, Select};
use flate2::{read::GzDecoder, write::GzEncoder};
use lzma::reader::LzmaReader;
use lzma::LzmaWriter;
use std::{
    env,
    fs::{self, File},
    io::{self, BufReader, Write},
    path::Path,
};
use tar::{Archive, Builder};

pub fn open_archive<P: AsRef<Path>>(
    filename: P,
    archive_type: ArchiveType,
) -> Archive<Box<dyn std::io::Read>> {
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

        ArchiveType::Bz2 => {
            let bzip2_reader = BzDecoder::new(file);
            Archive::new(Box::new(bzip2_reader) as Box<dyn std::io::Read>)
        }
        ArchiveType::Unknown => {
            eprintln!("Unknown archive type");
            std::process::exit(1);
        }
    }
}

pub fn list_archive_contents<P: AsRef<Path>>(filename: P) -> io::Result<()> {
    let archive_type = detect_archive_type(&filename);
    let file = File::open(&filename)?;
    let mut archive = match archive_type {
        ArchiveType::Gz => {
            let gz_decoder = GzDecoder::new(BufReader::new(file));
            Archive::new(Box::new(gz_decoder) as Box<dyn io::Read>)
        },
        ArchiveType::Xz => {
            let lzma_reader = LzmaReader::new_decompressor(file).expect("Failed to create LZMA decompressor");
            Archive::new(Box::new(lzma_reader) as Box<dyn io::Read>)
        },
        ArchiveType::Bz2 => {
            let bz_decoder = BzDecoder::new(BufReader::new(file));
            Archive::new(Box::new(bz_decoder) as Box<dyn io::Read>)
        },
        ArchiveType::Unknown => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported or unknown archive type")),
    };

    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?;
        println!("{}", path.display());
    }

    Ok(())
}

fn detect_archive_type<P: AsRef<Path>>(path: P) -> ArchiveType {
    match path.as_ref().extension().and_then(|ext| ext.to_str()) {
        Some("gz") => ArchiveType::Gz,
        Some("xz") => ArchiveType::Xz,
        Some("bz2") => ArchiveType::Bz2,
        _ => ArchiveType::Unknown,
    }
}

pub fn search_files(keyword: &str) -> Vec<String> {
    let current_dir = env::current_dir().expect("Failed to read current directory");
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                let file_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();
                if file_name.contains(keyword)
                    && (file_name.ends_with(".bz2")
                        || file_name.ends_with(".xz")
                        || file_name.ends_with(".gz"))
                {
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
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Dialoguer error: {}", err),
        )),
    }
}

pub fn handle_filename(sub_matches: &clap::ArgMatches) -> String {
    sub_matches
        .get_one::<String>("FILE")
        .unwrap_or_else(|| {
            eprintln!("File argument is missing");
            std::process::exit(1);
        })
        .clone()
}

fn add_files_recursively<P: AsRef<Path>>(
    tar: &mut Builder<Box<dyn Write>>,
    path: P,
    base_path: &Path,
    base_folder_name: &str,
    verbose: bool,
) -> Result<(), io::Error> {
    let path = path.as_ref();
    let relative_path = path.strip_prefix(base_path).unwrap_or(path);
    let tar_path = Path::new(base_folder_name).join(relative_path);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            add_files_recursively(tar, entry_path, base_path, base_folder_name, verbose)?;
        }
    } else {
        if verbose {
            println!("Adding file: {}", tar_path.display());
        }
        tar.append_path_with_name(path, tar_path)?;
    }

    Ok(())
}

pub fn create_archive_from_files<P: AsRef<Path>>(
    files: Vec<P>,
    filename: &str,
    verbose: bool,
    archive_type: ArchiveType,
) -> Result<(), io::Error> {
    let mut output_file: Option<String> = None;

    match archive_type {
        ArchiveType::Gz => {
            output_file = Some(format!("{}.tar.gz", filename));
        }
        ArchiveType::Xz => {
            output_file = Some(format!("{}.tar.xz", filename));
        }
        ArchiveType::Bz2 => {
            output_file = Some(format!("{}.tar.bz2", filename));
        }
        ArchiveType::Unknown => {
            eprintln!("Unknown archive type");
            std::process::exit(1);
        }
    }
    let file = File::create(output_file.as_ref().unwrap())?;
    let encoder: Box<dyn Write> = match archive_type {
        ArchiveType::Gz => Box::new(GzEncoder::new(file, flate2::Compression::default())),
        ArchiveType::Xz => Box::new(LzmaWriter::new_compressor(file, 6).unwrap()), // The '6' is the compression level
        ArchiveType::Bz2 => Box::new(BzEncoder::new(file, bzip2::Compression::default())),
        ArchiveType::Unknown => {
            eprintln!("Unknown archive type");
            std::process::exit(1);
        }
    };
    let mut tar = Builder::new(encoder);

    // Directly calculate base_path without causing a prolonged borrow
    let base_path = if let Some(first_file) = files.first() {
        first_file.as_ref().parent().unwrap_or_else(|| Path::new(""))
    } else {
        Path::new("")
    };

    // Use filename as the base folder name within the archive
    let base_folder_name = filename.trim();

    // Iterate over files without causing a borrow issue
    for file in &files {
        add_files_recursively(&mut tar, file.as_ref(), base_path, base_folder_name, verbose)?;
    }

    tar.finish()?;

    if verbose {
        println!("Archive created successfully: {:?}", &output_file.as_ref().unwrap());
    }

    Ok(())
}


pub fn extract_files<R: std::io::Read>(
    mut archive: Archive<R>,
    verbose: bool,
) -> Result<(), std::io::Error> {
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
