use std::fs;

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

// TODO: map is divided into fixed viewports. The viewport changes when player moves out of the
// current viewport

// pub(crate) fn update_viewport(view_port: &mut ViewPort, player: &mut Player) {
//   let left_boundary = view_port.x as f32 + view_port.width as f32 * 0.2;
//   let right_boundary = view_port.x as f32 + view_port.width as f32 * 0.4;
//
//   if player.x < left_boundary && view_port.x > 0 {
//     view_port.x -= 1;
//   }
//
//   if player.x > right_boundary {
//     view_port.x += 1;
//   }
// }
