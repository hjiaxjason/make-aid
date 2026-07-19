use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::fs::File;
use regex::Regex;

fn extract_includes(file_path: &Path) -> io::Result<HashSet<PathBuf>> {
    let mut includes = HashSet::new();

    let mut file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let re = Regex::new(r#"#include\s*(["<][^">]+[">])"#).unwrap();

    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 {
            break;
        }

        if re.is_match(line) {
            if let Some(captured) = re.captures(line) {
                if let Some(group1) = captured.get(1) {
                    includes.insert(group1.as_str());
                }
            }
        }
    }

    Ok(includes)
}

fn find_system_library(lib_name) -> io::Result<PathBuf> {

            
