// TODO: 1) Add ratatui to the system to get a bottom hotbar
//       2) Add block types like spikes and ladders - ladders done
//       3) Add start and finish points, maybe checkpoints too
mod app;
mod game;
mod ui;

use app::{App, AppState};
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

fn main() -> Result<(), io::Error> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;

  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app = App::new();
  let mut last_frame = Instant::now();

  loop {
    let now = Instant::now();
    let dt = (now - last_frame).as_secs_f32();
    last_frame = now;

    terminal.draw(|f| {
      app.render(f);
    })?;

    if event::poll(Duration::from_millis(0))? {
      if let Event::Key(key) = event::read()? {
        app.handle_key(key);
      }
    }

    // app.update(dt);

    if app.state == AppState::Quit {
      break;
    }

    std::thread::sleep(Duration::from_millis(16));
  }

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  terminal.show_cursor()?;

  Ok(())
}
