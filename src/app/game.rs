use std::fs;

use crate::app::{App, AppState, GameEvent};

impl App {
  // Updates game state if app state is playing
  pub fn update_game(&mut self, dt: f32) {
    #[allow(clippy::collapsible_if)]
    if let Some(game) = self.game.as_mut() {
      if let Some(event) = game.update(dt) {
        match event {
          GameEvent::Death => {
            if game.player.lives > 0 {
              self.state = AppState::Message(GameEvent::Death);
            } else {
              self.state = AppState::Message(GameEvent::Lost);
            }
          }
          GameEvent::Checkpoint => {
            self.state = AppState::Message(GameEvent::Checkpoint);
          }
          GameEvent::Won => {
            self.state = AppState::Message(GameEvent::Won);
          }
          _ => {}
        }
      }
    }
  }

  // Loads valid maps select in the map_select app state
  pub fn get_available_maps(&mut self) {
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
