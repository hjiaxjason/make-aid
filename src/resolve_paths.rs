//! The function of this file is resolving references, whether from Makefile prerequisites or
//! #includes into actual filesystem paths. There are two distinct resolution strategies: resolving
//! project-relative file, or external dependency (using pkg-config/system include paths).
use std::path::PathBuf;
use std::env;
use std::fs;

enum Dependency {
    LocalResolved(PathBuf),
    SystemResolved(PathBuf),
    Unresolved(String),
}

struct SearchPaths {
    system_paths: Vec<PathBuf>,
    local_paths: Vec<PathBuf>,
}

/// Given the header name of an includes, builds a PathBuf with the proper search path.
///
/// Parses delimiters ('<...>' or "...") to enforce preprocessing search priorities.
///
/// # Search Order
///
/// * **Angle Brackets (`<...>`)**
///   1. `pkg-config` directories declared explicitly in the Makefile.
///   2. OS-default fallback directories (e.g., `/usr/include`, `/opt/homebrew/include`).
///
/// * **Double Quotes (`"..."`)**
///   1. Local directory of the file currently being processed (e.g., `src/`). In this case we use
///      env::current_dir().
///   2. Global `include/` folder located at the project workspace root.
///   3. Fallback: Entirely defaults to the system search priority above.
///
/// # Edge Cases
/// * Returns `None` if the file cannot be located on disk across all search layers.
/// * Cleans and resolves relative traversal components like `../` before verification. 

fn build_search_paths(header_name: &str, current_file: &Path, project_root: &Path) -> Option<PathBuf> {
    
    if (path_str.starts_with("<") && path_str.ends_with(">")) {
        // system search

    } else if (path_str.starts_with("\"") && path_str.ends_with("\"")) {
        // local search first
        // first search current dir
        let parent = current_file.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(project_root);
        search_dir(parent, header_name);

        // search include directory at project root
        let include_path: PathBuf = project_root.join("include");
        search_dir(include_path, header_name);

    }
}

fn search_dir(dir_path: &PathBuf | &Path, target_name: &str) -> Option<PathBuf> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if file_name == target_name {
                    let file_path = PathBuf::from(target_name);
                    return file_path;
                }
            }
        }
    }
} 

/// Given the name of a library, attempts to find the the path from pkg-config.
fn get_pkg_config_dirs(lib_name: &str) -> Option<Vec<PathBuf>> {
    let output = Command::new("pkg-config")
        .args(["--cflags-only-I", lib_name])
        .output()
        .ok()?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);

        eprintln!("pkg-config failed for library '{}': {}", lib_name, err_msg.trim());
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(stdout.split_whitespace().filter_map(|tok| tok.strip_prefix("-I")).map(PathBuf::from).collect(), )
}

/// Based on operating system, get fallback directory name
fn get_fallback_dirs() -> Option<Vec<PathBuf>> {
    #[cfg(target_os = "linux")]
    {
        vec![PathBuf::from("/usr/lib/pkgconfig"), /* ... */]
    }
    #[cfg(target_os = "macos")]
    {
        vec![PathBuf::from("/opt/homebrew/lib/pkgconfig"), /* ... */]
    }
    #[cfg(target_os = "windows")]
    {
        vec![/* ... */]
    }
}

/// Searches fallback_dir for .pc file
fn get_pc_file(fallback_dir: Vec<PathBuf>, lib_name: &str) -> Option<PathBuf> {
    for path in &fallback_dir {
        let target_path = path.join("{lib_name}.pc");
        if fallback_dir.contains(&target_path) {
            return target_path
        }
    }
}

