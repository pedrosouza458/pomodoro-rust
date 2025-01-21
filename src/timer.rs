use std::{
    io,
    num::ParseIntError,
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::audio::play_audio;
use crate::terminal::clear_terminal;

pub fn set_timer(time: u64) {
    let mut count: i16 = 0;
    loop {
        count += 1;

        clear_terminal();
        println!("Time to focus!");

        let start_time = SystemTime::now();

        while start_time.elapsed().unwrap().as_secs() < time {
            sleep(Duration::new(1, 0));

            match start_time.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_secs() % 60 == 0 {
                        println!("{} focus minutes has passed", elapsed.as_secs() / 60);
                    }
                }
                Err(e) => {
                    println!("Error: {e}");
                    break;
                }
            }
        }

        play_audio();

        if count < 4 {
            set_rest(300);
        } else {
            set_rest(1800);
            let continue_timer: bool = restart_timer();
            if continue_timer == true {
                count = 0;
                continue;
            } else {
                break;
            }
        }
    }
}

pub fn set_rest(time: u64) {
    clear_terminal();
    println!("Time to rest");
    let start_time = SystemTime::now();

    while start_time.elapsed().unwrap().as_secs() < time {
        sleep(Duration::new(1, 0));

        match start_time.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_secs() % 60 == 0 {
                    println!("{} rest minutes has passed", elapsed.as_secs() / 60);
                }
            }
            Err(e) => {
                println!("Error: {e}");
                break;
            }
        }
    }
    play_audio();
}

pub fn restart_timer() -> bool {
    clear_terminal();
    println!(
        "Wanna continue the pomodoro? 
         1 - to yes
         0 - to no"
    );
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading user input");
    let santitazed_input: Result<i16, ParseIntError> = input.trim().parse();

    let result: bool = match santitazed_input {
        Ok(1) => true,
        _ => false,
    };
    result
}
