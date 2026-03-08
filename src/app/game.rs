use std::fs;

use crate::app::{App, AppState, MessageType};

impl App {
  // Updates game state if app state is playing
  pub fn update_game(&mut self, dt: f32) {
    if let Some(game) = self.game.as_mut() {
      game.update(dt);

      if !game.player.alive {
        if game.player.lives == 0 {
          self.state = AppState::Message(MessageType::Lost);
        } else {
          self.state = AppState::Message(MessageType::Death);
        }
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
