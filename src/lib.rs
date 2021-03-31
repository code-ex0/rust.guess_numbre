use std::time::{SystemTime, UNIX_EPOCH};
use std::io;
use std::io::{Write, stdin};
#[warn(dead_code)]
#[warn(unused_must_use)]

pub fn get_time_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub fn input(question: &str) -> String {
    let mut input_line = String::new();
    print!("{}",question);
    io::stdout().flush();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().to_string()
}

pub fn ask_number (question: &str) -> i32 {
    let mut input = String::new();
    print!("{}", question);
    io::stdout().flush();
    stdin().read_line(&mut input);
    return match input.trim().parse() {
        Ok(o) => {
            o
        },
        Err(_) => {
            -1
        }
    }
}