# bfcore

An interpreter for BrainF*ck without std or alloc.

## Why?

Now you can run brainf*ck on any hardware you want. This library doesn't even require a memory allocator!

## Usage

Here's the smallest example of how you'd run brainf*ck using bfcore.


```rust
extern crate bfcore;
use bfcore::{Interpreter, Input, Output};

#[derive(Default)]
struct In; impl Input for In {}
#[derive(Default)]
struct Out; impl Output for Out {}


fn main() {
    Interpreter::new(
        "+[----->+++<]>+.---.+++++++..+++.[--->+<]>-----.--[->++++<]>-.--------.+++.------.--------.-[--->+<]>.",
        In::default(),
        Out::default()
    ).run();
}
```

That doesn't really show you much, though.

Let me be more clear.

```rust
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
        "+[----->+++<]>+.---.+++++++..+++.[--->+<]>-----.--[->++++<]>-.--------.+++.------.--------.-[--->+<]>.",
        In::default(),
        Out::default()
    ).run();
}
```

The `Input` trait allows you to provide input to the interpreter, and the `Output` trait allows it to output to where ever you'd like.

You can also have the input and output objects maintain states, like a buffer.


```rust
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
        MyInput::default(),
        MyOutput::default()
    ).run(); // Run the interpreter
}
```

I don't have an example of this, but you could even use outputting non-printable characters to change the states of the input and output objects to do different things. You could have them switch between writing to files or to the screen by outputing special characters to switch modes, and more.


