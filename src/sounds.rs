use std::fs::File;
use std::io::BufReader;
use rodio::Source;

pub fn play_ding() {
    let device = rodio::default_output_device().unwrap();
    let file = File::open("ding.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    rodio::play_raw(&device, source.convert_samples());
}
