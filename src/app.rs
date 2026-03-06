pub mod game;
pub mod input;
pub mod render;
pub mod state;

use crate::game::game_main::Game;
pub use state::{AppState, MessageType};

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
