//TODO:   1) Add keys to collect in order to escape
//        6) Add animations between screens
//        7) Add colors to different tile types
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

  // panic handling
  let original_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    // Restore the terminal
    let _ = crossterm::terminal::disable_raw_mode();
    let _ = crossterm::execute!(
      std::io::stdout(),
      crossterm::terminal::LeaveAlternateScreen,
      crossterm::cursor::Show
    );
    // Print the actual panic message using the original hook
    original_hook(panic_info);
  }));

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
