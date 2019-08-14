extern crate bfcore;
use bfcore::{Input, Interpreter, Output};
use std::io::stdin;



/// Captures input from commandline as needed.
#[derive(Default)]
struct MyInput { buffer: String }
impl Input for MyInput {
    fn input(&mut self) -> char {
        // Only get user input if we've run out of characters in our buffer
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
    // Create an interpreter with a program that prints hello world
    // Give it instances of our input and output structs
    Interpreter::new(
        r#"+[----->+++<]>+.---.+++++++..+++.[--->+<]>-----.--[->++++<]>-.--------.+++.------.--------.-[--->+<]>."#,
        &mut MyInput::default(),
        &mut MyOutput::default()
    ).run(); // Run the interpreter
}
