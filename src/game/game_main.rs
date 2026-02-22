use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

use crate::game::input;
use crate::game::map;
use crate::game::physics;
use crate::game::player;
use crate::game::renderer;

use std::error::Error;
use std::path::Path;

pub(crate) struct Game {
  map: Vec<Vec<map::Tile>>,
  view_port: map::ViewPort,
  pub(crate) player: player::Player,
  pub(crate) map_name: String,
}

impl Game {
  pub fn new(map_path: &str) -> Result<Self, Box<dyn Error>> {
    let map = map::load_map(map_path)?;

    let view_port = map::ViewPort {
      x: 0,
      width: 190,
      height: 43,
    };

    let player = player::Player {
      x: 5.0,
      y: map.len() as f32 - 5.0,
      vx: 0.0,
      vy: 0.0,
      on_ground: false,
      climbing: false,
      climb_cooldown: 0.0,
      lives: 3,
    };

    let map_name = Path::new(map_path)
      .file_name()
      .and_then(|name| name.to_str())
      .unwrap_or("unknown")
      .to_string();

    Ok(Self {
      map,
      view_port,
      player,
      map_name,
    })
  }

  pub fn update(&mut self, dt: f32) {
    player::update_player_properties(&mut self.player, &self.map, dt);

    physics::apply_physics(&mut self.player, &self.map, dt);

    map::update_viewport(&mut self.view_port, &self.player);
  }

  pub fn handle_input(&mut self, key: KeyEvent, dt: f32) {
    input::handle_input(&mut self.player, key, dt, &self.map);
  }

  pub fn render(&self, f: &mut Frame, area: Rect) {
    let lines = renderer::build_frame_lines(&self.map, &self.view_port, &self.player);

    let paragraph = Paragraph::new(lines);

    f.render_widget(paragraph, area);
  }
}
