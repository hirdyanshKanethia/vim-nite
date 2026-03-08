use ratatui::{Frame, widgets::Clear};

use crate::app::{App, AppState, MessageType};

mod checkpoint;
mod death;
mod layout;
mod lost;

pub fn render(f: &mut Frame, app: &App) {
  let message_type = match app.state {
    AppState::Message(m) => m,
    _ => return,
  };

  let area = layout::centered_rect(50, 25, f.area());

  // clear background
  f.render_widget(Clear, area);

  match message_type {
    MessageType::Death => death::render(f, area),
    MessageType::Checkpoint => checkpoint::render(f, area),
    MessageType::Lost => lost::render(f, area),
  }
}
