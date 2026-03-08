use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::state::AppState, game::game_main::Game};

use super::App;
use super::MessageType;

impl App {
  pub fn handle_key(&mut self, key: KeyEvent, dt: f32) {
    match self.state {
      AppState::MainMenu => self.handle_main_menu_input(key),
      AppState::MapSelect => self.handle_map_select_input(key),
      AppState::Playing => {
        if let Some(game) = &mut self.game {
          match game.handle_input(key, dt) {
            Ok(false) => {}
            Ok(true) => {
              self.ui.selected_index = 0;
              self.state = AppState::Paused;
            }
            Err(_) => self.state = AppState::Quit,
          }
        }
      }
      AppState::Message(MessageType::Lost) => self.handle_message_input_lost(key),
      AppState::Message(_) => self.handle_message_input(key),
      AppState::Paused => self.handle_pause_menu_input(key),
      _ => {}
    }
  }

  fn handle_main_menu_input(&mut self, key: KeyEvent) {
    match key.code {
      KeyCode::Char('k') => {
        if self.ui.selected_index > 0 {
          self.ui.selected_index -= 1;
        }
      }
      KeyCode::Char('j') => {
        if self.ui.selected_index < 1 {
          self.ui.selected_index += 1;
        }
      }
      KeyCode::Enter => match self.ui.selected_index {
        0 => {
          // map select
          self.load_maps();
          self.state = AppState::MapSelect;
        }
        1 => self.state = AppState::Quit,
        _ => {}
      },
      _ => {}
    }
  }

  fn handle_map_select_input(&mut self, key: KeyEvent) {
    match key.code {
      KeyCode::Char('k') => {
        if self.ui.selected_index > 0 {
          self.ui.selected_index -= 1;
        }
      }
      KeyCode::Char('j') => {
        if self.ui.selected_index + 1 < self.available_maps.len() {
          self.ui.selected_index += 1;
        }
      }
      KeyCode::Enter => {
        if let Some(map_name) = self.available_maps.get(self.ui.selected_index) {
          let path = format!("./maps/{}", map_name);

          if let Ok(game) = Game::new(&path) {
            self.game = Some(game);
            self.state = AppState::Playing;
          }
        }
      }
      KeyCode::Esc => {
        self.state = AppState::MainMenu;
      }
      _ => {}
    }
  }

  fn handle_pause_menu_input(&mut self, key: KeyEvent) {
    match key.code {
      KeyCode::Char('k') => {
        if self.ui.selected_index > 0 {
          self.ui.selected_index -= 1;
        }
      }
      KeyCode::Char('j') => {
        if self.ui.selected_index < 1 {
          self.ui.selected_index += 1;
        }
      }
      KeyCode::Enter => match self.ui.selected_index {
        0 => {
          self.state = AppState::Playing; // Resume
        }
        1 => {
          self.game = None;
          self.state = AppState::MainMenu; // Back to main
        }
        _ => {}
      },
      KeyCode::Esc => {
        self.state = AppState::Playing;
      }
      _ => {}
    }
  }

  fn handle_message_input(&mut self, key: KeyEvent) {
    match key.code {
      KeyCode::Enter => {
        if let Some(game) = &mut self.game {
          game.respawn_player();
          self.state = AppState::Playing;
        }
      }
      _ => {}
    }
  }

  fn handle_message_input_lost(&mut self, key: crossterm::event::KeyEvent) {
    match key.code {
      KeyCode::Enter => {
        self.game = None;
        self.ui.selected_index = 0;
        self.state = AppState::MainMenu;
      }
      _ => {}
    }
  }
}
