use std::fs;

use crate::game::player::Player;

pub(crate) const CLIMB_COOLDOWN: f32 = 0.25;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Tile {
  Empty,
  Wall,
  // PlayerSpawn,
  // later:
  // Spike,
  Ladder,
  // Water,
}

pub(crate) struct TileProperties {
  pub solid: bool,
  pub deadly: bool,
  pub climbable: bool,
  pub standable: bool,
}

pub(crate) struct ViewPort {
  pub(crate) x: usize,
  pub(crate) width: usize,
  pub(crate) height: usize,
}

// Tile implementations. Tiles possess certain properties like solid, deadly, climbable, standable
// that define their behaviour
impl Tile {
  pub(crate) fn from_char(c: char) -> Self {
    match c {
      ' ' => Tile::Empty,
      '█' => Tile::Wall,
      // '>' => Tile::PlayerSpawn,
      '#' => Tile::Ladder,
      _ => Tile::Empty,
    }
  }

  pub(crate) fn to_char(self) -> char {
    match self {
      Tile::Empty => ' ',
      Tile::Wall => '█',
      // Tile::PlayerSpawn => '>',
      Tile::Ladder => '#',
    }
  }

  pub(crate) fn properties(self) -> TileProperties {
    match self {
      Tile::Empty => TileProperties {
        solid: false,
        deadly: false,
        climbable: false,
        standable: false,
      },
      Tile::Wall => TileProperties {
        solid: true,
        deadly: false,
        climbable: false,
        standable: true,
      },
      Tile::Ladder => TileProperties {
        solid: false,
        deadly: false,
        climbable: true,
        standable: false,
      },
    }
  }
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

// Updates the viewport to fixed positions when player coordinates move out of veiwport
pub(crate) fn update_viewport(view_port: &mut ViewPort, player: &Player) {
  if player.x > view_port.x as f32 + view_port.width as f32 {
    view_port.x += view_port.width;
  }

  if player.x < view_port.x as f32 {
    view_port.x -= view_port.width;
  }
}
