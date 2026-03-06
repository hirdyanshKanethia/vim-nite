use std::fs;

use crate::game::game_main::Game;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
  Frame,
  layout::{Constraint, Direction},
};

// -----------------------------------------
// Central hub for TUI logic and navigation
// -----------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
  MainMenu,
  MapSelect,
  Playing,
  Paused,
  Message(MessageType),
  Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
  Checkpoint,
  Death,
}

pub struct UiState {
  pub selected_index: usize,
}

pub struct App {
  pub state: AppState,
  pub ui: UiState,
  pub game: Option<Game>,
  pub available_maps: Vec<String>,
}

impl App {
  pub fn new() -> Self {
    Self {
      state: AppState::MainMenu,
      ui: UiState { selected_index: 0 },
      game: None,
      available_maps: Vec::new(),
    }
  }
}

impl App {
  // ----------------
  // INPUT HANDLING
  // ----------------
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
      KeyCode::Up => {
        if self.ui.selected_index > 0 {
          self.ui.selected_index -= 1;
        }
      }
      KeyCode::Down => {
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

  // ------------------
  // Rendering handler
  // ------------------

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

        crate::ui::message::render(f, self);
      }

      AppState::Quit => {}
    }
  }

  // Updates game state if app state is playing
  pub fn update_game(&mut self, dt: f32) {
    if let Some(game) = self.game.as_mut() {
      game.update(dt);

      if !game.player.alive {
        self.state = AppState::Message(MessageType::Death);
      }
    }
  }

  // Loads valid maps select in the map_select app state
  pub fn load_maps(&mut self) {
    self.available_maps.clear();

    if let Ok(entries) = fs::read_dir("./maps") {
      for entry in entries.flatten() {
        let path = entry.path();

        if let Some(ext) = path.extension()
          && ext == "txt"
          && let Some(name) = path.file_name()
          && let Some(name_str) = name.to_str()
        {
          self.available_maps.push(name_str.to_string());
        }
      }
    }

    self.available_maps.sort();
    self.ui.selected_index = 0;
  }
}
