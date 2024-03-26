use std::{fs::OpenOptions, io::Write};
use chrono::{self, Local};

pub fn log(text: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./log")
        .unwrap();
    writeln!(file, "{}: {}", Local::now().format("%Y-%m-%d %H:%M"), text).unwrap();
}

#[macro_export]
macro_rules! log {
    ($($args: tt)*) => {
        logger::log(format!($($args)*))
    }
}
