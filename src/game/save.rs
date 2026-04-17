use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MapStats {
  pub best_time_ms: u64, // Storing as ms is easier for JSON than a raw Duration
  pub completions: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SaveData {
  pub maps: HashMap<String, MapStats>,
}

impl SaveData {
  fn get_save_path() -> PathBuf {
    // This looks for/creates a path based on (qualifier, organization, application)
    // On Linux: ~/.local/share/vim-nite/
    if let Some(proj_dirs) = ProjectDirs::from("com", "hirdyansh", "vim-nite") {
      let data_dir = proj_dirs.data_dir();
      // Ensure the directory exists (mkdir -p)
      let _ = fs::create_dir_all(data_dir);
      return data_dir.join("stats.json");
    }
    // Fallback to local directory if something goes wrong
    PathBuf::from("stats.json")
  }

  pub fn load() -> Self {
    let path = Self::get_save_path();
    fs::read_to_string(path)
      .ok()
      .and_then(|content| serde_json::from_str(&content).ok())
      .unwrap_or_default()
  }

  pub fn save(&self) -> std::io::Result<()> {
    let path = Self::get_save_path();
    let json = serde_json::to_string_pretty(self).unwrap();
    fs::write(path, json)
  }

  pub fn update_best_time(&mut self, map_name: String, new_time: std::time::Duration) {
    let new_ms = new_time.as_millis() as u64;
    let stats = self.maps.entry(map_name).or_insert(MapStats {
      best_time_ms: u64::MAX, // Start with max so any time is better
      completions: 0,
    });

    if new_ms < stats.best_time_ms {
      stats.best_time_ms = new_ms;
    }
    stats.completions += 1;
  }
}
