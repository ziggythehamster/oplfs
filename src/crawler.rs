use ascii::*;
use compress_tools::uncompress_archive_file;
use diesel::sqlite::SqliteConnection;
use owo_colors::OwoColorize;
use scan_dir::ScanDir;
use system_cnf_parser::SystemCnf;

use std::error::Error;
use std::fs::{File, DirEntry};
use std::path::PathBuf;

/// A list of acceptable file extensions
const ACCEPTABLE_EXTENSIONS: [&str; 2] = [
    ".iso",
    ".iso9660"
];

/// Walks the passed directory and finds discs
pub fn crawl(_connection: &SqliteConnection, path_prefix: &PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Crawling {} ...", path_prefix.to_string_lossy().blue());

    if !path_prefix.as_path().exists() {
        return Err(format!("Path {} does not exist", path_prefix.to_string_lossy()).into());
    }

    ScanDir::files().walk(path_prefix, |walker| {
        for (entry, _) in walker.filter(has_acceptable_extension) {
            let path = &entry.path();
            let path = path.to_string_lossy();

            // Fetch the title ID
            let title_id = match get_title_id(&entry) {
                Ok(x) => x,
                Err(e) => {
                    println!("{} {}: {}", "[error]".red(), path.magenta(), e);
                    continue;
                }
            };

            println!("{} {} {}",
                "[new disc]".green(),
                format!("[{}]", title_id).yellow(),
                path.magenta()
            )
        }

        Ok(())
    }).unwrap() // ideally we handled all errors in the iterator, or else this will panic
}

/// Reads the title ID from a disc image
fn get_title_id(entry: &DirEntry) -> Result<String, Box<dyn Error>> {
    let path = entry.path();

    let mut source = File::open(&path)?;
    let mut target = Vec::default();

    uncompress_archive_file(&mut source, &mut target, "SYSTEM.CNF")?;

    let system_cnf = AsciiString::from_ascii(target)?;
    let system_cnf = SystemCnf::from(system_cnf);

    match system_cnf.title_id() {
        Some(x) => Ok(x.to_string()),
        None          => Err("no title ID found".into())
    }
}

/// Filter that checks if the file has an acceptable extension or not
fn has_acceptable_extension(file: &(DirEntry, String)) -> bool {
    let name = &file.1;

    ACCEPTABLE_EXTENSIONS.iter().any(|&ext| name.ends_with(ext))
}
