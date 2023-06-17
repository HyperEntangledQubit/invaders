#[allow(unused_imports)]

use glob::glob;
// use std::error::Error;
use rusty_audio::Audio;

fn main(){// -> Result <(), Box<dyn Error>> {
    let mut _audio = Audio::new();

    for audiofile in glob("audio/*.wav").expect("Failed to find wav file") {
        match audiofile {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}
