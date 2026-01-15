# rls - Rust ls Clone

A full-featured, modular `ls` clone written in Rust. This project replicates and extends the functionality of the classic Unix `ls` command, providing directory listings with color, formatting, sorting, and more.

## Features
- Short and long listing formats
- Colored output by file type
- Human-readable file sizes
- Show hidden files
- Sorting (name, size, time)

# rls

A modular Rust implementation of the `ls` command, supporting advanced features and incremental development. (Updated January 2026)

## Features

- Modular code structure
- Argument parsing with `clap`
- Long and short listing formats
- Colorized output
- Human-readable file sizes
- Hidden file support
- Sorting and reverse sorting
- Recursive directory listing
- File type indicators
- User/group lookup
- Grid layout
- File type icons

## Usage

```sh
rls [OPTIONS] [PATH]
```

### Options

- `-l` : Long format (shows permissions, owner, group, size, date, name)
- `-H` : Human-readable sizes (e.g., 1K, 234M)
- `-a` : Show hidden files
- `--sort <field>` : Sort by field (`name`, `size`, `time`)
- `-r` : Reverse sort order
- `-R` : Recursive listing
- `--icons` : Show file type icons

## Example

```sh
rls -lH --sort size --icons /path/to/dir
```

## Documentation

See docs/USAGE.md for detailed usage and docs/FEATURES.md for feature explanations.

## License

MIT

## Project Structure
- `src/main.rs` - Entry point, argument parsing, calls main logic
- `src/lib/args.rs` - Command-line argument definitions
- `src/lib/color.rs` - File coloring and type indicator logic
- `src/functions/list_dir.rs` - Directory listing implementation

## Documentation
See `DOCS.md` for detailed code documentation and developer notes.

---

© 2026 Mark Maher Eweida (marco5dev). MIT License.
