use crate::game::{self, game_loop::Game};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
  MainMenu,
  MapSelect,
  Playing,
  Paused,
  Quit,
}

pub struct UiState {
  pub selected_index: usize,
}

pub struct App {
  pub state: AppState,
  pub ui: UiState,
  pub game: Option<Game>,
}

impl App {
  pub fn new() -> Self {
    Self {
      state: AppState::MainMenu,
      ui: UiState { selected_index: 0 },
      game: None,
    }
  }
}

impl App {
  pub fn handle_key(&mut self, key: KeyEvent) {
    match self.state {
      AppState::MainMenu => self.handle_main_menu_input(key),
      // AppState::Playing => {
      //   if let Some(game) = &mut self.game {
      //     game::input::handle_input(self.game?);
      //   }
      // }
      _ => {}
    }
  }

  fn handle_main_menu_input(&mut self, key: KeyEvent) {
    match key.code {
      KeyCode::Up => {
        if self.ui.selected_index > 0 {
          self.ui.selected_index -= 1;
        }
      }
      KeyCode::Down => {
        if self.ui.selected_index < 1 {
          self.ui.selected_index += 1;
        }
      }
      KeyCode::Enter => match self.ui.selected_index {
        0 => {
          // Start game
          if let Ok(game) = Game::new("./maps/map1.txt") {
            self.game = Some(game);
            self.state = AppState::Playing;
          }
        }
        1 => self.state = AppState::Quit,
        _ => {}
      },
      _ => {}
    }
  }

  // pub fn update(&mut self, dt: f32) {
  //   if self.state == AppState::Playing {
  //     if let Some(game) = &mut self.game {
  //       game.update(dt);
  //     }
  //   }
  // }

  pub fn render(&self, f: &mut Frame) {
    match self.state {
      AppState::MainMenu => crate::ui::render_main_menu(f, self),

      // AppState::MapSelect => crate::ui::render_map_select(f, self),
      AppState::MapSelect => {}

      AppState::Playing => {
        if let Some(game) = &self.game {
          // game.render(f);
        }
      }

      AppState::Paused => {
        if let Some(game) = &self.game {
          // game.render(f); // draw game first
          // crate::ui::render_pause(f, self); // overlay pause UI
        }
      }

      AppState::Quit => {}
    }
  }
}
