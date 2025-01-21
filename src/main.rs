use std::io;
use std::num::ParseIntError;

mod audio;
mod terminal;
mod timer;

use terminal::clear_terminal;
use timer::set_timer;

fn main() {
    clear_terminal();
    println!(
        r"          
_________________________________________________________                                                
|  _ __   ___  _ __ ___   ___   __| | ___  _ __ ___     |
|  | '_ \ / _ \| '_ ` _ \ / _ \ / _` |/ _ \| '__/ _ \   |
|  | |_) | (_) | | | | | | (_) | (_| | (_) | | | (_) |  |
|  | .__/ \___/|_| |_| |_|\___/ \__,_|\___/|_|  \___/   |
|  |_|                                                  |
_________________________________________________________
A pomodoro 🍅 built in Rust    

1 - to start timer
0 - to exit                 
         "
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
