//TODO:   1) Add keys to collect in order to escape
//        2) Add error handling for valid and invalid maps
//  DONE  3) The hotbar currently disappears when message state is achieved, fix that i need the
//           hotbar there :-(
//  DONE  4) Add a bottom blank line, and make some sort of mechanism for it so that ; character
//           triggers a state where everything typed is gone there and entering prints a funny
//           message there like "duh, you thought something would happend. ts never works. are you
//           new here?"
//        5) Add map metadata saving (probably in json) to save data like best completion time
//        6) If possible add an animation in the starting of the game
//        7) Add colors in ts 
//        8) Add music in ts
mod app;
mod game;
mod timer;
mod ui;

use app::App;
use app::state::AppState;
use crossterm::{
  event::{self, Event},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
  io,
  time::{Duration, Instant},
};

// logging crates
use simplelog::*;
use std::fs::File;

fn main() -> Result<(), io::Error> {
  WriteLogger::init(
    LevelFilter::Debug,
    Config::default(),
    File::create("game.log").unwrap(),
  )
  .unwrap();

  log::info!("Game started");
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;

  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app = App::new();
  let mut last_frame = Instant::now();
  let frame_duration = Duration::from_millis(17);

  // Main app loop
  loop {
    let frame_start = Instant::now();

    let dt = (frame_start - last_frame).as_secs_f32();
    last_frame = frame_start;

    terminal.draw(|f| {
      app.render(f);
    })?;

    #[allow(clippy::collapsible_if)]
    if event::poll(Duration::from_millis(0))? {
      if let Event::Key(key) = event::read()? {
        app.handle_key(key, dt);
        // log::debug!("dt value: {:?}", dt);
      }
    }

    if app.state == AppState::Playing {
      app.update_game(dt);
    } else if app.state == AppState::Quit {
      break;
    }

    // Sleeps the thread for the rest of the frame duration
    let frame_time = frame_start.elapsed();
    if frame_time < frame_duration {
      std::thread::sleep(frame_duration - frame_time);
    }
  }

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  terminal.show_cursor()?;

  Ok(())
}
