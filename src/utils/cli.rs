use terminal_size::{Width, terminal_size};
use crate::version;

pub fn print_centered_header(text: &str) {
    let width = match terminal_size() {
        Some((Width(w), _)) => w as usize,
        None => 80, // fallback width
    };

    let total_text = format!(" {} ", text); // add space padding
    let text_len = total_text.len();
    let rem = if width > text_len { width - text_len } else { 0 };

    let half = rem / 2;
    let line = format!(
        "{}{}{}",
        "=".repeat(half),
        total_text,
        "=".repeat(width - text_len - half)
    );

    println!("{}", line);
}

pub fn header() {
    print_centered_header("Avi Core");
    println!("{}", "Version: ".to_owned() + version::VERSION);
    println!("Embrace, Inc, by {}", version::AUTHOR);
    println!("{}", "Build Date: ".to_owned() + version::BUILD_DATE);
}