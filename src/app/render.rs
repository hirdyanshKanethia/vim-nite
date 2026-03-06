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
        if let Some(game) = &self.game {
          let chunks = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(f.area());

          let game_area = chunks[0];
          let hotbar_area = chunks[1];

          game.render(f, game_area);
          crate::ui::render_hotbar(f, hotbar_area, self);
        }
      }

      AppState::Paused => {
        if let Some(game) = &self.game {
          let chunks = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(f.area());

          let game_area = chunks[0];
          let hotbar_area = chunks[1];

          game.render(f, game_area);
          crate::ui::render_hotbar(f, hotbar_area, self);

          crate::ui::render_pause_menu(f, self);
        }
      }

      AppState::Message(_) => {
        if let Some(game) = &self.game {
          game.render(f, f.area());
        }

        crate::ui::render_message(f, self);
      }

      AppState::Quit => {}
    }
  }
}
