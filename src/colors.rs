use colored::*;

pub struct ColoredOutput;

impl ColoredOutput {
    pub fn success(msg: &str) -> ColoredString {
        msg.green().bold()
    }

    pub fn error(msg: &str) -> ColoredString {
        msg.red().bold()
    }

    pub fn info(msg: &str) -> ColoredString {
        msg.cyan()
    }

    pub fn key(key: &str) -> ColoredString {
        key.bright_blue().bold()
    }

    pub fn value(value: &str) -> ColoredString {
        value.green()
    }

    pub fn path(path: &str) -> ColoredString {
        path.yellow()
    }

    pub fn database(db: &str) -> ColoredString {
        db.magenta().bold()
    }

    pub fn count(count: usize) -> ColoredString {
        count.to_string().cyan().bold()
    }

    pub fn header(text: &str) -> ColoredString {
        text.bright_cyan().bold()
    }

    pub fn warning(msg: &str) -> ColoredString {
        msg.yellow().bold()
    }

    pub fn format_key_value(key: &str, value: &str) -> String {
        format!("{} {} {}",
            Self::key(key),
            "=".bright_black(),
            Self::value(value)
        )
    }

    pub fn format_env_line(key: &str, value: &str) -> String {
        format!("{}={}", Self::key(key), Self::value(value))
    }
}
