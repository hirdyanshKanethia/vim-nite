// TODO: 1) Add ratatui to the system to get a bottom hotbar - DONE
//       2) Add block types like spikes and ladders - both DONE
//       3) Add start and finish points, maybe checkpoints - checkpoints DONE
//   new-4) Add timer to make speedrunning the main motive
mod app;
mod game;
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

fn main() -> Result<(), io::Error> {
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
