use std::io::{self, Write};

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap()
}

pub fn read_input() -> i32 {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input value");
    if let Ok(option) = input.trim().parse() {
        return option;
    }
    0
}

pub fn prompt_restart() {
    clear_terminal();
    println!(
        "Wanna continue the pomodoro?
         1 - to yes
         0 - to no",
    );
}
