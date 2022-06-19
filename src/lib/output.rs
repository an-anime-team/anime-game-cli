use std::str::FromStr;

use colorful::Colorful;

pub fn notice<T: ToDisplayable>(msg: T) {
    println!(" {} {}", " & ".bg_blue().white().bold(), msg.to_string());
}

pub fn warn<T: ToDisplayable>(msg: T) {
    println!(" {} {}", " ! ".bg_yellow().white().bold(), msg.to_string());
}

pub fn error<T: ToDisplayable>(msg: T) {
    println!(" {} {}", " ! ".bg_light_red().white().bold(), msg.to_string());
}

pub trait ToDisplayable {
    fn to_string(&self) -> String;
}

impl ToDisplayable for &str {
    fn to_string(&self) -> String {
        String::from_str(self).unwrap()
    }
}

impl ToDisplayable for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}

impl<T> ToDisplayable for Vec<T> where T: ToString {
    fn to_string(&self) -> String {
        let mut output = self[0].to_string();

        for line in &self[1..] {
            output += &format!("\n     {}", line.to_string());
        }

        output
    }
}
