use std::fs;

use crate::player::Player;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Tile {
  Empty,
  Wall,
  PlayerSpawn,
  // later:
  // Spike,
  // Ladder,
  // Water,
}

impl Tile {
  pub(crate) fn from_char(c: char) -> Self {
    match c {
      ' ' => Tile::Empty,
      '█' => Tile::Wall,
      '>' => Tile::PlayerSpawn,
      _ => Tile::Empty,
    }
  }

  pub(crate) fn to_char(self) -> char {
    match self {
      Tile::Empty => ' ',
      Tile::Wall => '█',
      Tile::PlayerSpawn => '>',
    }
  }

  pub(crate) fn is_solid(self) -> bool {
    matches!(self, Tile::Wall)
  }
}

pub(crate) struct ViewPort {
  pub(crate) x: usize,
  pub(crate) width: usize,
  pub(crate) height: usize,
}

pub(crate) fn load_map(path: &str) -> std::io::Result<Vec<Vec<Tile>>> {
  let map_text = fs::read_to_string(path)?;

  Ok(
    map_text
      .lines()
      .map(|line| line.chars().map(Tile::from_char).collect())
      .collect(),
  )
}

pub(crate) fn update_viewport(view_port: &mut ViewPort, player: &Player) {
  if player.x > view_port.x as f32 + view_port.width as f32 {
    view_port.x += view_port.width;
  }

  if player.x < view_port.x as f32 {
    view_port.x -= view_port.width;
  }
}
