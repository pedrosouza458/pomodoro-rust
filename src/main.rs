use std::io;
use std::num::ParseIntError;

mod audio;
mod timer;
mod terminal;

use timer::set_timer;
use terminal::clear_terminal;

fn main() {
    clear_terminal();
    println!(
        "
        Pomodoro Timer 🍅
        1 - to set your timer
        0 - to quit"
    );

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input value");
    let sanitazed_input: Result<i32, ParseIntError> = input.trim().parse();

    match sanitazed_input {
        Ok(1) => set_timer(1500),
        _ => {}
    }
}
