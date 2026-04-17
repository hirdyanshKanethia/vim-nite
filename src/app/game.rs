use std::fs;

use crate::{
  app::{App, AppState, Event},
  game::save::SaveData,
};

impl App {
  // Updates game state if app state is playing
  pub fn update_game(&mut self, dt: f32) {
    #[allow(clippy::collapsible_if)]
    if let Some(game) = self.game.as_mut() {
      if let Some(event) = game.update(dt) {
        match event {
          Event::PlayerDeath => {
            if game.player.lives > 0 {
              self.state = AppState::Message(Event::PlayerDeath);
            } else {
              self.state = AppState::Message(Event::GameLost);
            }
          }
          Event::GameCheckpointReached => {
            self.state = AppState::Message(Event::GameCheckpointReached);
          }
          Event::GameWon => {
            let mut save_data = SaveData::load();

            save_data.update_best_time(game.map_name.clone(), game.timer.elapsed());

            let _ = save_data.save();

            self.state = AppState::Message(Event::GameWon);
          }
          _ => {}
        }
      }
    }
  }

  pub fn get_available_maps(&mut self) {
    self.available_maps.clear();

    if let Ok(entries) = fs::read_dir("./maps") {
      for entry in entries.flatten() {
        let path = entry.path();

        #[allow(clippy::collapsible_if)]
        if let Some(name_str) = path.file_name().and_then(|n| n.to_str()) {
          if name_str.ends_with(".map.txt") {
            self.available_maps.push(name_str.to_string());
          }
        }
      }
    }

    self.available_maps.sort();
    self.ui.selected_index = 0;
  }
}
