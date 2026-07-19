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

fn build_search_paths(header_name: &str) -> Option<PathBuf> {
    if (path_str.starts_with("<") && path_str.ends_with(">")) {
        // system search
    } else if (path_str.starts_with("\"") && path_str.ends_with("\"")) {
        // local search first, then system
        // first search current dir
        let current_dir = env::current_dir()?;

        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if file_name == header_name {
                        let file_path = PathBuf::from(file_name);
                        return file_path;
                    }
                }
            }
        }

        // then search user include paths
    }
}
