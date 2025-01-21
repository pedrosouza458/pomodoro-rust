use rodio::{source::Source, Decoder, OutputStream};
use std::{
    fs::File,
    io::BufReader,
};

pub fn play_audio() -> () {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("./assets/bell-sound.wav").unwrap());
    let source = Decoder::new(file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
}
