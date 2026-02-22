use crossterm::event::KeyEvent;

use crate::game::input;
use crate::game::map;
use crate::game::physics;
use crate::game::player;
use crate::game::player::update_player_properties;
use crate::game::renderer;

use std::error::Error;
use std::thread;
use std::time::Duration;

pub struct Game {
  map: Vec<Vec<map::Tile>>,
  view_port: map::ViewPort,
  player: player::Player,
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

    Ok(Self {
      map,
      view_port,
      player,
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
}
