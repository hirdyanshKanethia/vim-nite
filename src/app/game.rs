use std::fs;

use crate::{
  app::{App, AppState, Event},
  game::{map::MapInfo, save::SaveData},
};

impl App {
  /// Updates game state if app state is playing
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

  /// updates the list of available maps in App struct
  /// available maps refers to maps that pass the is_map_valid() function check
  pub fn get_available_maps(&mut self) {
    self.available_maps.clear();
    let save_data = SaveData::load();

    if let Ok(entries) = fs::read_dir("./maps") {
      for entry in entries.flatten() {
        let path = entry.path();

        #[allow(clippy::collapsible_if)]
        if let Some(name_str) = path.file_name().and_then(|n| n.to_str()) {
          if name_str.ends_with(".map.txt") {
            let path_str = format!("./maps/{}", name_str);
            let is_valid = crate::game::map::is_map_valid(&path_str);
            let stats = save_data.maps.get(name_str);

            self.available_maps.push(MapInfo {
              name: name_str.to_string(),
              is_valid,
              best_time_ms: stats.map(|s| s.best_time_ms),
              completions: stats.map(|s| s.completions).unwrap_or(0),
            });
          }
        }
      }
    }

    self.available_maps.sort_by(|a, b| a.name.cmp(&b.name));
    self.ui.selected_index = 0;
  }
}
