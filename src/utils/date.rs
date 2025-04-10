
#[macro_export]
macro_rules! current_date {
    () => {{
        chrono::Local::now().format("%Y-%m-%d").to_string()
    }};
}