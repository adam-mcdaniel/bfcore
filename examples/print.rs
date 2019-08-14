
extern crate bfcore;
use bfcore::{Interpreter, Input, Output};

#[derive(Default)]
struct In;
impl Input for In {
    fn input(&mut self) -> char {
        // When the interpreter needs user input, return EOF, or '\0'
        '\0'
    }
}

#[derive(Default)]
struct Out;
impl Output for Out {
    fn output(&mut self, ch: char) {
        // When the interpreter wants to output, print the output character
        print!("{}", ch);
    }
}


fn main() {
    Interpreter::new(
        "+[------->++<]>++.++.---------.+++++.++++++.[---->+<]>+++.+[->+++<]>++.[--->+<]>+.--[->+++<]>-.++++++++++++.+++.----.-------.--[--->+<]>...",
        In::default(),
        Out::default()
    ).run();
}