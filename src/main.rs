// use glob::glob;
use spaceinvaders::frame::new_frame;
use spaceinvaders::render;

use crossterm::{
  cursor::{Hide, Show},
  event::{self, Event, KeyCode},
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use terminal::stdout;

use std::sync::mpsc;
use std::thread;
use std::io;
use std::error::Error;
use std::time::Duration;
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {
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

  // Terminal
  let mut stdout = stdout();
  enable_raw_mode()?;
  stdout.execute(EnterAlternateScreen)?;
  stdout.execute(Hide)?;

  // Rendering gameloop in alt thread
  let (render_tx, render_rx) = mpsc::channel();
  let render_handle = thread::spawn(move || {
    let mut last_frame = new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    loop {
      match render_rx.recv() {
        Ok(x) => x,
        Err(_) => break,
      };
      let curr_frame = new_frame();
      render::render(&mut stdout, &last_frame, &curr_frame, false);
      last_frame = curr_frame;
    }
  });

  // Game loop
  'gameloop: loop {
    // Per-frame init
    let curr_frame = new_frame();

    while event::poll(Duration::default())? {
      if let Event::Key(key_event) = event::read()? {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
              audio.play("lose");
              break 'gameloop;
            }
            _ => {}
         }
      }
    }
    // Draw and render
    // Buffer channel here because the receiving channel wont be ready
    // at same time transmit channel is.
    let _ = render_tx.send(curr_frame);
    thread::sleep(Duration::from_millis(1)); // Only gen 1000 frames/s
  }

  // cleanup
  drop(render_tx);
  render_handle.join().unwrap();

  audio.wait();

  stdout.execute(Show)?;
  stdout.execute(LeaveAlternateScreen)?;
  disable_raw_mode()?;

  Ok(())
}
