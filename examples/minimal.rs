extern crate bfcore;
use bfcore::{Interpreter, Input, Output};

#[derive(Default)]
struct In; impl Input for In {}
#[derive(Default)]
struct Out; impl Output for Out {}


fn main() {
    Interpreter::new(
        "+[----->+++<]>+.---.+++++++..+++.[--->+<]>-----.--[->++++<]>-.--------.+++.------.--------.-[--->+<]>.",
        &mut In::default(),
        &mut Out::default()
    ).run();
}