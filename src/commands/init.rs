use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use clap::Parser;
use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn find_files(args: Cli) {
    let mut map: HashMap<PathBuf, HashSet<PathBuf>> = HashMap::new();
    let valid_exts = ["c","h","cpp","hpp"];
    for entry in WalkDir::new(&args.path) {
        let entry = match {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Error reading entry: {}", err);
                continue;
            }
        };

        let path: &Path = entry.path();

        if valid_exts.iter().any(|x| x = path.extension()) {
            println!(path.filename());
        }
    }
}

fn extract_includes(file_path: &Path) -> io::Result<HashSet<PathBuf>> {
    let mut includes = HashSet::new();

    let mut file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let re = Regex::new(r#"#include\s*["<]([^">]+)[">]"#).unwrap();

    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 {
            break;
        }

        if re.is_match(line) {
            let captured = re.captures(line).unwrap();
            includes.insert(captured.get(1).as_str());
        }
    }

    Ok(includes)
}
            
