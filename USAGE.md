
# rls - How to Use

## Building

1. Ensure you have Rust and Cargo installed.
2. Clone or download this repository.
3. In the project directory, run:
   ```sh
   cargo build
   ```

## Running

Run the program with:
```sh
cargo run -- [OPTIONS] [PATH]
```

- `[PATH]` is the directory to list (default: current directory)
- `[OPTIONS]` are any combination of the following:

### Options
- `-l`, `--long`           Use long format (detailed info: permissions, owner, group, size, date, name)
- `-H`, `--human`          Human-readable sizes (e.g., 1.2K, 3.4M)
- `-a`, `--all`            Show hidden files (files starting with .)
- `--sort <name|size|time>` Sort by name, size, or time (default: name)
- `-r`, `--reverse`        Reverse sort order
- `-R`, `--recursive`      List subdirectories recursively
- `--icons`                Show file type icons

### Examples

List current directory:
```
cargo run --
```

List a directory in long format:
```
cargo run -- -l /etc
```

Show all files, human sizes, sorted by size, with icons:
```
cargo run -- -a -H --sort size --icons
```

Recursively list all files, reverse order:
```
cargo run -- -R -r
```

---
For developer documentation, see `DOCS.md`.
