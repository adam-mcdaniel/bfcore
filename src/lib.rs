#![no_std]
use core::mem::size_of;

/// 16 bit instruction pointer
const TAPE_SIZE: usize = 65535;
/// How many nested loops allowed
const NESTED_LOOP_LIMIT: usize = 1024;

/// Change this to u16 for 16 bit cell support
type Cell = u8;
/// Calculate the MAX_SIZE for a Cell based on its number of bytes
const MAX_SIZE: Cell = ((1 << (size_of::<Cell>() * 8)) - 1) as Cell;

/// Trait for getting a character
pub trait Input {
    fn input(&mut self) -> char {
        '\0'
    }
}

/// Trait for outputing a character
pub trait Output {
    fn output(&mut self, _: char) {}
}

/// The struct containing the data for interpretting brainfuck code
pub struct Interpreter<I, O>
where
    I: Input,
    O: Output,
{
    // The object used to get input
    input: I,
    // The object used to output
    output: O,

    // The pointer to the current data cell
    data_ptr: usize,
    // The tape of data cells
    data_tape: [Cell; TAPE_SIZE],
    // The pointer to the current instruction in the instruction tape
    instruction_ptr: usize,
    // The instruction tape
    instruction_tape: [Cell; TAPE_SIZE],

    // The current number of nested loops
    nested_loop_counter: usize,
    // The array of nested loop addresses
    nested_loop_stack: [usize; NESTED_LOOP_LIMIT],
}

/// Create an instruction tape from a &str
/// Basically, it has an array of 2^16 slots for characters,
/// and fills them in as needed with the chars from the str.
/// The interpreter stops when it hits a '\0'.
fn instruction_tape_from_str(s: &str) -> [Cell; TAPE_SIZE] {
    let mut tape = [0; TAPE_SIZE];

    for (i, ch) in s.chars().enumerate() {
        tape[i] = ch as Cell;
    }

    tape
}

impl<I, O> Interpreter<I, O>
where
    I: Input,
    O: Output,
{
    /// Create a new interpreter from a program, an input object, and an output object
    pub fn new(program: &str, input: I, output: O) -> Self {
        Self {
            input,
            output,

            data_ptr: 0,
            data_tape: [0; TAPE_SIZE],

            instruction_ptr: 0,
            // Create instruction tape from program
            instruction_tape: instruction_tape_from_str(program),

            nested_loop_counter: 0,
            nested_loop_stack: [0; NESTED_LOOP_LIMIT],
        }
    }

    /// Resets the current interpreter.
    /// Resets the instruction and data
    /// pointers, loop counter and stack,
    /// and the data tape.
    fn reset(&mut self) {
        self.data_ptr = 0;
        self.data_tape = [0; TAPE_SIZE];
        self.instruction_ptr = 0;

        self.nested_loop_counter = 0;
        self.nested_loop_stack = [0; NESTED_LOOP_LIMIT];
    }

    /// Execute the program.
    /// This can be done over and over again, the interpreter is reset each time.
    /// HOWEVER: State kept within your input and output objects CANNOT be reset!!!
    ///          This only resets the interpreter's internal state before execution!!!!
    pub fn run(&mut self) {
        // Reset interpreter state
        self.reset();

        // While the current instruction is not zero
        while self.instruction_tape[self.instruction_ptr] != 0 {
            // convert the instruction to a character
            let ins = self.instruction_tape[self.instruction_ptr] as u8 as char;
            match ins {
                '[' => self.enter_loop(),
                ']' => self.exit_loop(),
                '+' => self.increment(),
                '-' => self.decrement(),
                '>' => self.right(),
                '<' => self.left(),
                '.' => self.output(),
                ',' => self.input(),
                _ => {}
            }

            // Move to next instruction
            self.instruction_ptr += 1;
        }
    }

    /// Increments the value in the current data cell.
    /// Rust does not support overflow, so this automatically wraps.
    fn increment(&mut self) {
        self.data_tape[self.data_ptr] = if self.data_tape[self.data_ptr] == MAX_SIZE {
            0
        }
        // If the current value is MAX, wrap
        else {
            self.data_tape[self.data_ptr] + 1
        } // otherwise increment normally
    }

    /// Decrements the value in the current data cell.
    /// Rust does not support overflow, so this automatically wraps.
    fn decrement(&mut self) {
        self.data_tape[self.data_ptr] = if self.data_tape[self.data_ptr] == 0 {
            MAX_SIZE
        }
        // If the current value is zero, wrap
        else {
            self.data_tape[self.data_ptr] - 1
        } // otherwise decrement normally
    }

    /// Decrements the data cell pointer.
    /// If the pointer is zero, wrap around.
    fn left(&mut self) {
        if self.data_ptr == 0 {
            self.data_ptr = TAPE_SIZE - 1;
        } else {
            self.data_ptr -= 1;
        }
    }

    /// Increments the data cell pointer.
    /// If the pointer is MAX, wrap around.
    fn right(&mut self) {
        if self.data_ptr == TAPE_SIZE - 1 {
            self.data_ptr = 0;
        } else {
            self.data_ptr += 1;
        }
    }

    /// Call the Output object's output method using the value of the current data cell
    fn output(&mut self) {
        self.output
            .output(self.data_tape[self.data_ptr] as u8 as char)
    }

    /// Store the result of the Input object's input method in the current data cell
    fn input(&mut self) {
        self.data_tape[self.data_ptr] = self.input.input() as Cell
    }

    /// Move the instruction pointer to the topmost loop on the stack
    fn goto_topmost_loop(&mut self) {
        self.instruction_ptr = self.nested_loop_stack[self.nested_loop_counter - 1];
    }

    /// If the current cell is zero, go to the end of this loop
    /// Otherwise, enter the loop and push this instruction pointer value
    /// onto the top of the nested loop stack!
    fn enter_loop(&mut self) {
        // If the current data cell is not zero
        if self.data_tape[self.data_ptr] != 0 {
            // Then push this instruction pointer onto the top of the nested loop stack
            self.nested_loop_stack[self.nested_loop_counter] = self.instruction_ptr;
            self.nested_loop_counter += 1;
        } else {
            // Else, jump to the matching closing loop
            let mut loop_counter = 1;
            for (i, ins) in self.instruction_tape[self.instruction_ptr + 1..]
                .iter()
                .enumerate()
            {
                match *ins as u8 as char {
                    '[' => loop_counter += 1,
                    ']' => loop_counter -= 1,
                    _ => {}
                }
                if loop_counter == 0 {
                    // Jump to the instruction AFTER the closing loop
                    self.instruction_ptr = i + self.instruction_ptr + 1;
                    break;
                }
            }
        }
    }

    /// If the value of the current cell is zero,
    ///     continue and pop off the last instruction
    ///     pointer value from the nested loop stack
    /// Otherwise, jump to the topmost loop instruction
    ///     pointer value on the nested loop stack.
    fn exit_loop(&mut self) {
        // If the current cell is not zero, goto the most recent loop beginning
        if self.data_tape[self.data_ptr] != 0 {
            self.goto_topmost_loop()
        } else {
            // If the current cell is zero, pop off the last instruction
            // pointer value from the nested loop stack.
            self.nested_loop_counter -= 1;
            self.nested_loop_stack[self.nested_loop_counter] = 0;
        }
    }
}
