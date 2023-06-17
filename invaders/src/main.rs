#[allow(unused_imports)]

// use glob::glob;
use std::error::Error;
use rusty_audio::Audio;

fn main(){// -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();

    // Replace these files with the actual sounds
    audio.add("explode", "audio/explode.wav");
    audio.add("lose", "audio/lose.wav");
    audio.add("move", "audio/move.wav");
    audio.add("pew", "audio/pew.wav");
    audio.add("startup", "audio/startup.wav");
    audio.add("win", "audio/win.wav");

    // Using glob package to find files
    // for audiofile in glob("audio/*.wav").expect("Failed to find wav file") {
    //     match audiofile {
    //         Ok(path) => audio.add(
    //             path.file_stem().unwrap(),
    //             path.file_name().unwrap()
    //         ),

    //         Err(e) => println!("{:?}", e),
    //     }
    // }
    audio.play("startup");

    // cleanup
    audio.wait();
}
