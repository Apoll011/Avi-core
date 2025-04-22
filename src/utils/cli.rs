use std::io;
use std::io::Write;
use terminal_size::{Width, terminal_size};
use crate::version;

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

pub fn print_centered_header(text: &str) {
    let width = match terminal_size() {
        Some((Width(w), _)) => w as usize,
        None => 40,
    };

    let total_text = format!(" {} ", text);
    let text_len = total_text.len();
    let rem = width.saturating_sub(text_len);

    let half = rem / 2;
    let line = format!(
        "{}{}{}",
        "=".repeat(half),
        if text_len > 2 { total_text } else { "==".to_string() },
        "=".repeat(width - text_len - half)
    );

    println!("{}", line);
}

pub fn header() {
    print_centered_header("Avi Core");
    println!("{}", "Version: ".to_owned() + version::VERSION);
    println!("Embrace, Inc. By {}.", version::AUTHOR);
    println!("{}", "Build Date: ".to_owned() + version::BUILD_DATE);
    println!("Copyright (C) 2025");
    print_centered_header("");
}