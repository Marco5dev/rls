use colored::*;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;

pub fn color_and_indicator(name: &str, meta: &fs::Metadata, show_icons: bool) -> String {
    let file_type = meta.file_type();
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    let mode = if cfg!(unix) {
        meta.permissions().mode()
    } else {
        0
    };
    // Icon map for common file types/extensions
    let icon = if !show_icons { "" } else if file_type.is_dir() {
        "📁"
    } else if file_type.is_symlink() {
        "🔗"
    } else if cfg!(unix) && (mode & 0o111 != 0) {
        "⚙️"
    } else if file_type.is_socket() {
        "🔌"
    } else if file_type.is_fifo() {
        "🚰"
    } else if file_type.is_block_device() {
        "💾"
    } else if file_type.is_char_device() {
        "⌨️"
    } else if name.ends_with(".tar") || name.ends_with(".zip") || name.ends_with(".gz") || name.ends_with(".bz2") || name.ends_with(".xz") || name.ends_with(".7z") {
        "📦"
    } else if name.ends_with(".jpg") || name.ends_with(".jpeg") || name.ends_with(".png") || name.ends_with(".gif") || name.ends_with(".bmp") || name.ends_with(".svg") {
        "🖼️"
    } else if name.ends_with(".md") {
        "📝"
    } else if name.ends_with(".json") {
        "🧾"
    } else if name.ends_with(".toml") || name.ends_with(".yml") || name.ends_with(".yaml") {
        "⚙️"
    } else if name.ends_with(".html") || name.ends_with(".htm") {
        "🌐"
    } else if name.ends_with(".css") {
        "🎨"
    } else if name.ends_with(".rs") {
        "🦀"
    } else if name.ends_with(".py") {
        "🐍"
    } else if name.ends_with(".js") {
        "📜"
    } else if name.ends_with(".ts") {
        "📘"
    } else if name.ends_with(".jsx") {
        "⚛️"
    } else if name.ends_with(".tsx") {
        "🔷"
    } else if name.ends_with(".c") {
        "🔵"
    } else if name.ends_with(".cpp") {
        "🔷"
    } else if name.ends_with(".h") || name.ends_with(".hpp") {
        "📗"
    } else if name.ends_with(".java") {
        "☕"
    } else if name.ends_with(".go") {
        "🐹"
    } else if name.ends_with(".sh") {
        "🐚"
    } else if name.ends_with(".php") {
        "🐘"
    } else if name.ends_with(".rb") {
        "💎"
    } else if name.ends_with(".swift") {
        "🕊️"
    } else if name.ends_with(".kt") {
        "🤖"
    } else if name.ends_with(".pl") {
        "🦪"
    } else if name.ends_with(".lua") {
        "🌙"
    } else if name.ends_with(".dart") {
        "🎯"
    } else if name.ends_with(".scala") {
        "🔺"
    } else if name.ends_with(".cs") {
        "⚙️"
    } else if name.ends_with(".sql") {
        "🗄️"
    } else if name.ends_with(".xml") {
        "📰"
    } else if name.ends_with(".vue") {
        "🖼️"
    } else {
        ""
    };

    // Color logic (as before)
    let icon_space = if show_icons && !icon.is_empty() { format!("{} ", icon) } else { String::new() };
    if file_type.is_dir() {
        format!("{}{}{}", icon_space, name.blue().bold(), "/")
    } else if file_type.is_symlink() {
        format!("{}{}{}", icon_space, name.cyan(), "@")
    } else if cfg!(unix) && (mode & 0o111 != 0) {
        format!("{}{}{}", icon_space, name.green(), "*")
    } else if file_type.is_socket() {
        format!("{}{}{}", icon_space, name.magenta(), "=")
    } else if file_type.is_fifo() {
        format!("{}{}{}", icon_space, name.yellow(), "|")
    } else if file_type.is_block_device() {
        format!("{}{}{}", icon_space, name.bright_yellow(), "#")
    } else if file_type.is_char_device() {
        format!("{}{}{}", icon_space, name.bright_yellow(), "%")
    } else if name.ends_with(".tar") || name.ends_with(".zip") || name.ends_with(".gz") || name.ends_with(".bz2") || name.ends_with(".xz") || name.ends_with(".7z") {
        format!("{}{}", icon_space, name.red())
    } else if name.ends_with(".jpg") || name.ends_with(".jpeg") || name.ends_with(".png") || name.ends_with(".gif") || name.ends_with(".bmp") || name.ends_with(".svg") {
        format!("{}{}", icon_space, name.bright_magenta())
    } else if name.ends_with(".rs") || name.ends_with(".c") || name.ends_with(".cpp") || name.ends_with(".h") || name.ends_with(".hpp") || name.ends_with(".py") || name.ends_with(".js") || name.ends_with(".ts") || name.ends_with(".java") || name.ends_with(".go") || name.ends_with(".sh") {
        format!("{}{}", icon_space, name.bright_cyan())
    } else {
        format!("{}{}", icon_space, name.white())
    }
}
