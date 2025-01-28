mod audio;
mod terminal;
mod timer;

use terminal::{clear_terminal, prompt_restart, read_input};
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

    while let 1 = read_input() {
        set_timer(25);
        prompt_restart();
    }
}
