extern crate bfcore;
use bfcore::{Input, Interpreter, Output};
use std::io::stdin;

#[derive(Default)]
struct MyInput {
    buffer: String,
}

impl Input for MyInput {
    fn input(&mut self) -> char {
        if self.buffer.is_empty() {
            stdin()
                .read_line(&mut self.buffer)
                .expect("Did not enter a correct string");
        }

        let result = self.buffer.chars().nth(0);
        if !self.buffer.is_empty() {
            self.buffer = self.buffer[1..].to_string();
        }

        match result {
            Some(ch) => ch,
            None => 0 as char,
        }
    }
}

#[derive(Default)]
struct MyOutput;

impl Output for MyOutput {
    fn output(&mut self, ch: char) {
        print!("{}", ch);
    }
}

fn main() {
    Interpreter::new(
        r#"+[----->+++<]>+.---.+++++++..+++.[--->+<]>-----.--[->++++<]>-.--------.+++.------.--------.-[--->+<]>."#,
        MyInput::default(),
        MyOutput::default()
    ).run();
}
