use colored::Colorize;

pub fn error(message: &str) {
    println!("{}\n", message.red().bold())
}
