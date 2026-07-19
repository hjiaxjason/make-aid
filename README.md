# make-aid

First-aid kit for Makefiles. This is a Rust CLI tool to clean, heal, lint and generate structured build pipelines.

## Commands:
- **lint**: Audits existing Makefile. Detect missing headers, broken paths, and syntax issues, such as when a file is deleted or renamed.
- **heal**: Safely fix compiler warnings and prune dangling outputs. The heal subcommand deletes references to deleted files from a Makefile's dependency list, and compares missing files to newly untracked files and asks the user if they renamed it. If the user says Yes, rewrites Makefile with the updated file name.
- **undeclared-deps**: Reports headers included in source code but not declared in Makefile.

## Setup:
Built using Rust. To clone and run:
```bash
cargo build --release

