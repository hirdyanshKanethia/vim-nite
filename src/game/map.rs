use std::fs;

use crate::game::player::Player;

pub(crate) const VIEWPORT_HEIGHT: i32 = 43;
pub(crate) const VIEWPORT_WIDTH: i32 = 190;

pub(crate) struct ViewPort {
  pub(crate) x: usize,
  pub(crate) y: usize,
  pub(crate) width: usize,
  pub(crate) height: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Direction {
  Up,
  Down,
  Right,
  Left,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Tile {
  Empty,
  Wall,
  Spike(Direction),
  Ladder,
  Checkpoint,
  PlayerStart,
}

pub(crate) struct TileProperties {
  pub solid: bool, // solid property means that the player model cannot phase into the tile
  pub deadly: bool, // what could it be?
  pub climbable: bool, // The player can climb up or down when phased into this tile
  pub standable: bool, // The player can stand on top of this tile, may or may not be able to phase
  // into it. The main purpose of this is with ladders that I can think of, the player can phase
  // into the ladder block but can stand on it too.
  pub respawn: bool, // The tile is a respawn point, the player can respawn near this tile
}

// Tile implementations. Tiles possess certain properties like solid, deadly, climbable, standable
// that define their behaviour
impl Tile {
  pub(crate) fn from_char(c: char) -> Self {
    match c {
      ' ' => Tile::Empty,
      '█' => Tile::Wall,
      '#' => Tile::Ladder,
      '▲' => Tile::Spike(Direction::Up),
      '▼' => Tile::Spike(Direction::Down),
      '◀' => Tile::Spike(Direction::Left),
      '▶' => Tile::Spike(Direction::Right),
      '§' => Tile::Checkpoint,
      '@' => Tile::PlayerStart,
      _ => Tile::Empty,
    }
  }

  pub(crate) fn to_char(self) -> char {
    match self {
      Tile::Empty => ' ',
      Tile::Wall => '█',
      Tile::Ladder => '#',
      Tile::Spike(Direction::Up) => '▲',
      Tile::Spike(Direction::Down) => '▼',
      Tile::Spike(Direction::Left) => '◀',
      Tile::Spike(Direction::Right) => '▶',
      Tile::Checkpoint => '§',
      Tile::PlayerStart => '@',
    }
  }

  pub(crate) fn properties(self) -> TileProperties {
    match self {
      Tile::Empty => TileProperties {
        solid: false,
        deadly: false,
        climbable: false,
        standable: false,
        respawn: false,
      },
      Tile::Wall => TileProperties {
        solid: true,
        deadly: false,
        climbable: false,
        standable: true,
        respawn: false,
      },
      Tile::Ladder => TileProperties {
        solid: false,
        deadly: false,
        climbable: true,
        standable: false,
        respawn: false,
      },
      Tile::Spike(_) => TileProperties {
        solid: false,
        deadly: true,
        climbable: false,
        standable: false,
        respawn: false,
      },
      Tile::Checkpoint => TileProperties {
        solid: false,
        deadly: false,
        climbable: false,
        standable: true,
        respawn: true,
      },
      Tile::PlayerStart => TileProperties {
        solid: false,
        deadly: false,
        climbable: false,
        standable: false,
        respawn: false,
      },
    }
  }
}

pub(crate) fn load_map(path: &str) -> std::io::Result<(Vec<Vec<Tile>>, (usize, usize))> {
  let map_text = fs::read_to_string(path)?;

  let mut start = None;

  let map: Vec<Vec<Tile>> = map_text
    .lines()
    .enumerate()
    .map(|(y, line)| {
      line
        .chars()
        .enumerate()
        .map(|(x, c)| {
          if c == '@' {
            start = Some((x, y));
          }
          Tile::from_char(c)
        })
        .collect()
    })
    .collect();

  let start = start.expect("Map must contain atleast one start point");

  Ok((map, start))
}

// Updates the viewport to fixed positions when player coordinates move out of veiwport
pub(crate) fn update_viewport(view_port: &mut ViewPort, player: &Player) {
  if player.x >= view_port.x as f32 + view_port.width as f32 {
    view_port.x += view_port.width;
  }

  if player.x < view_port.x as f32 {
    view_port.x = view_port.x.saturating_sub(view_port.width);
  }

  if player.y >= view_port.y as f32 + view_port.height as f32 {
    view_port.y += view_port.height;
  }

  if player.y < view_port.y as f32 {
    view_port.y = view_port.y.saturating_sub(view_port.height);
  }
}
