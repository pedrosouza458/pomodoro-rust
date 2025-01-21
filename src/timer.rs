use std::{thread::sleep, time::Duration};

use crate::audio::play_audio;
use crate::terminal::clear_terminal;

static REPEAT: i32 = 4;

pub fn set_timer(minutes: u64) {
    for _ in 0..REPEAT {
        clear_terminal();
        wait(minutes, "focus");
        play_audio();
        set_rest(5);
    }
    set_rest(30);
}

pub fn set_rest(minutes: u64) {
    clear_terminal();
    println!("Time to rest");
    wait(minutes, "rest");
    play_audio();
}

fn wait(minutes: u64, kind: &str) {
    for i in 1..=minutes {
        sleep(Duration::from_secs(60));
        println!("{} {} minutes have passed", i, kind);
    }
}
