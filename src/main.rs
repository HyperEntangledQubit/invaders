use spaceinvaders::frame::{new_frame, Drawable};
use spaceinvaders::render;
use spaceinvaders::player::Player;
use spaceinvaders::invaders::Invaders;
use spaceinvaders::score::Score;
use spaceinvaders::level::Level;

use crossterm::{
  cursor::{Hide, Show},
  event::{self, Event, KeyCode},
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use std::sync::mpsc;
use std::thread;
use std::io;
use std::error::Error;
use std::time::Duration;
use std::time::Instant;
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

  audio.play("startup");

  // Terminal
  let mut stdout = io::stdout();
  enable_raw_mode()?;
  stdout.execute(EnterAlternateScreen)?;
  stdout.execute(Hide)?;

  // Rendering gameloop in alt thread
  let (render_tx, render_rx) = mpsc::channel();
  let render_handle = thread::spawn(move || {
    let mut last_frame = new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
		while let Ok(curr_frame) = render_rx.recv() {
      render::render(&mut stdout, &last_frame, &curr_frame, false);
      last_frame = curr_frame;
		}
    loop {
      match render_rx.recv() {
        Ok(x) => x,
        Err(_) => break,
      };
    }
  });

  // Game loop
  let mut player = Player::new();
	let mut instant = Instant::now();
	let mut invaders = Invaders::new();
	let mut score = Score::new();
	let mut level = Level::new();

  'gameloop: loop {
    // Per-frame init
		let delta = instant.elapsed();
		instant = Instant::now();
    let mut curr_frame = new_frame();

    while event::poll(Duration::default())? {
      if let Event::Key(key_event) = event::read()? {
        match key_event.code {
          KeyCode::Left => player.move_left(),
          KeyCode::Right => player.move_right(),
					KeyCode::Char(' ') | KeyCode::Enter => {
						if player.shoot() {
							audio.play("pew");
						}
					}
          KeyCode::Esc | KeyCode::Char('q') => {
            audio.play("lose");
            break 'gameloop;
          }
          _ => {}
         }
      }
    }
		// Updates
		player.update(delta);
		if invaders.update(delta) {
			audio.play("move");
		}
		let hits: u16 = player.detect_hits(&mut invaders);
		if hits > 0 {
			audio.play("explode");
			score.add_points(hits);
		}
    // Draw and render
		let drawables: Vec<&dyn Drawable> = vec![&player, &invaders, &score, &level];
		for drawable in drawables {
			drawable.draw(&mut curr_frame);
		}
    // Buffer channel here because the receiving channel wont be ready
    // at same time transmit channel is.
    let _ = render_tx.send(curr_frame);
    thread::sleep(Duration::from_millis(1)); // Only gen 1000 frames/s

		// Win or Lose
		if invaders.all_killed() {
			if level.increment_level() {
				audio.play("win");
				break 'gameloop;
			}
			invaders = Invaders::new();
		} else if invaders.reached_bottom() {
			audio.play("lose");
			break 'gameloop;
		}
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
