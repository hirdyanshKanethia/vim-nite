use ratatui::Frame;
use ratatui::layout::{Constraint, Direction};

use crate::app::state::AppState;

use super::App;

impl App {
  pub fn render(&self, f: &mut Frame) {
    match self.state {
      AppState::MainMenu => crate::ui::render_main_menu(f, self),

      AppState::MapSelect => crate::ui::render_map_select(f, self),

      AppState::Playing => {
        self.render_game_with_hotbar(f);
      }

      AppState::Paused => {
        self.render_game_with_hotbar(f);
        crate::ui::render_pause_menu(f, self);
      }

      AppState::Message(super::Event::InvalidMap) => {
        crate::ui::render_message(f, self);
      }

      AppState::Message(_) => {
        self.render_game_with_hotbar(f);
        crate::ui::render_message(f, self);
      }

      AppState::EnteringCommand => {
        self.render_game_with_hotbar(f);
      }

      AppState::Quit => {}
    }
  }

  // Function to render the hotbar on the bottom of the screen while playing
  fn render_game_with_hotbar(&self, f: &mut Frame) {
    if let Some(game) = &self.game {
      let chunks = ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints([
          Constraint::Min(0),
          Constraint::Length(1),
          Constraint::Length(1),
        ])
        .split(f.area());

      let game_area = chunks[0];
      let hotbar_area = chunks[1];
      let command_area = chunks[2];

      game.render(f, game_area);
      crate::ui::render_hotbar(f, hotbar_area, command_area, self);
    }
  }
}
