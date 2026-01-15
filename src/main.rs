use clap::Parser;
mod lib {
    pub mod args;
    pub mod color;
}
mod functions {
    pub mod collect_entries;
    pub mod file_entry;
    pub mod list_dir;
}
use crate::lib::args::Args;
use std::path::PathBuf;
use crate::functions::collect_entries::collect_entries;
use crate::functions::list_dir::render_entries;
fn main() {
    let args = Args::parse();
    let dir = PathBuf::from(&args.path);
    list_recursive(&dir, &args, None);
}

fn list_recursive(path: &PathBuf, args: &Args, prefix: Option<&str>) {
    let entries = collect_entries(path, args);
    render_entries(path, args, &entries, prefix);
    if args.recursive {
        for entry in &entries {
            if entry.metadata.is_dir() && entry.name != "." && entry.name != ".." {
                let mut sub_path = path.clone();
                sub_path.push(&entry.name);
                println!();
                list_recursive(&sub_path, args, Some(&sub_path.to_string_lossy()));
            }
        }
    }
}
