
# rls - Code Documentation

This document provides an overview of the code structure and key components of the `rls` project. (Updated January 2026)

## Modules

### 1. `lib/args.rs`
Defines the `Args` struct using `clap` for command-line argument parsing. All user options (long, human, all, sort, reverse, recursive, icons, path) are defined here.

### 2. `lib/color.rs`
Contains the `color_and_indicator` function, which determines the color and indicator symbol for each file based on its type and extension. Uses the `colored` crate for terminal colors and supports file type icons.

### 3. `functions/list_dir.rs`
Implements the `list_dir` function, which performs the directory listing. Handles:
- Reading directory entries
- Filtering hidden files
- Sorting and reversing
- Formatting output (long/grid)
- Recursion for subdirectories
- User/group lookup (via `users` crate)
- Human-readable sizes (via `humansize` crate)
- Terminal width detection (via `term_size` crate)
- File type icons (if enabled)

## Main Flow
- `main.rs` parses arguments and calls `list_dir` with the target path and options.
- `list_dir` prints the directory contents according to the options, using `color_and_indicator` for formatting and icons.

## Extending
- Add new features by creating a new file in `src/functions/` and updating `main.rs` as needed.
- Core logic and shared utilities should go in `src/lib/`.

## Dependencies
- `clap` for argument parsing
- `colored` for terminal colors
- `humansize` for human-readable file sizes
- `users` for user/group name lookup
- `chrono` for date formatting
- `term_size` for terminal width

---
For usage instructions, see `USAGE.md`.
For features and options, see `README.md`.
