use ratatui::{Frame, widgets::Clear};

use crate::app::{App, AppState, Event};

mod checkpoint;
mod death;
mod invalid_map;
mod layout;
mod lost;
mod won;

pub fn render(f: &mut Frame, app: &App) {
  let message_type = match app.state {
    AppState::Message(m) => m,
    _ => return,
  };

  let area = layout::centered_rect(50, 30, f.area());

  // clear background
  f.render_widget(Clear, area);

  match message_type {
    Event::InvalidMap => invalid_map::render(f, area),
    Event::PlayerDeath => death::render(f, area),
    Event::GameCheckpointReached => checkpoint::render(f, area),
    Event::GameLost => lost::render(f, area),
    Event::GameWon => won::render(f, area),
  }
}
