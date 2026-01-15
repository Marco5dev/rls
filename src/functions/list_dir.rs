use crate::lib::args::Args;
use crate::lib::color::color_and_indicator;
use super::file_entry::FileEntry;
use std::path::PathBuf;
use users::{get_group_by_gid, get_user_by_uid};

pub fn render_entries(
    _path: &PathBuf,
    args: &Args,
    entries: &[FileEntry],
    prefix: Option<&str>,
) {
    if let Some(pfx
    ) = prefix {
        println!("{}:", pfx);
    }
    if args.long {
        for entry in entries {
            let display_name = color_and_indicator(&entry.name, &entry.metadata, args.icons);
            let hard_links = 1;
            #[cfg(unix)]
            {
                use std::os::unix::fs::{MetadataExt, PermissionsExt};
                let mode = entry.metadata.permissions().mode();
                let perm_str = format!(
                    "{}{}{}{}{}{}{}{}{}",
                    if mode & 0o400 != 0 { 'r' } else { '-' },
                    if mode & 0o200 != 0 { 'w' } else { '-' },
                    if mode & 0o100 != 0 { 'x' } else { '-' },
                    if mode & 0o040 != 0 { 'r' } else { '-' },
                    if mode & 0o020 != 0 { 'w' } else { '-' },
                    if mode & 0o010 != 0 { 'x' } else { '-' },
                    if mode & 0o004 != 0 { 'r' } else { '-' },
                    if mode & 0o002 != 0 { 'w' } else { '-' },
                    if mode & 0o001 != 0 { 'x' } else { '-' },
                );
                let uid = entry.metadata.uid();
                let gid = entry.metadata.gid();
                let user = get_user_by_uid(uid)
                    .and_then(|u| u.name().to_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| uid.to_string());
                let group = get_group_by_gid(gid)
                    .and_then(|g| g.name().to_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| gid.to_string());
                let size = entry.metadata.len();
                let size_str = if args.human {
                    humansize::format_size(size, humansize::DECIMAL)
                } else {
                    size.to_string()
                };
                let mtime = entry.metadata.mtime();
                let datetime = {
                    use chrono::{Local, TimeZone};
                    let dt = Local.timestamp_opt(mtime, 0).single();
                    match dt {
                        Some(dt) => dt.format("%b %d %H:%M").to_string(),
                        None => "?".to_string(),
                    }
                };
                println!(
                    "{}{} {:>2} {:>8} {:>8} {:>8} {} {}",
                    if entry.metadata.is_dir() { 'd' } else { '-' },
                    perm_str,
                    hard_links,
                    user,
                    group,
                    size_str,
                    datetime,
                    display_name
                );
            }
            #[cfg(not(unix))]
            {
                let perm_str = "---------";
                let size = entry.metadata.len();
                let size_str = if args.human {
                    humansize::format_size(size, humansize::DECIMAL)
                } else {
                    size.to_string()
                };
                println!(
                    "{}{} {:>2} {:>5} {:>5} {:>8} {} {}",
                    if entry.metadata.is_dir() { 'd' } else { '-' },
                    perm_str,
                    hard_links,
                    "?",
                    "?",
                    size_str,
                    "?",
                    display_name
                );
            }
        }
    } else {
        use term_size;
        let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
        let max_len = entries
            .iter()
            .map(|e| color_and_indicator(&e.name, &e.metadata, args.icons).len())
            .max()
            .unwrap_or(0)
            + 2;
        let cols = if max_len == 0 { 1 } else { term_width / max_len }.max(1);
        let mut col = 0;
        for entry in entries {
            let display_name = format!(
                "{:<width$}",
                color_and_indicator(&entry.name, &entry.metadata, args.icons),
                width = max_len
            );
            print!("{}", display_name);
            col += 1;
            if col >= cols {
                println!();
                col = 0;
            }
        }
        if col != 0 {
            println!();
        }
    }
}