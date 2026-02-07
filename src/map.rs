use std::fs;

use crate::player::Player;

pub(crate) struct ViewPort {
  pub(crate) x: usize,
  pub(crate) width: usize,
}

pub(crate) fn load_map(path: &str) -> std::io::Result<Vec<Vec<char>>> {
  let map_text = fs::read_to_string(path)?;

  Ok(
    map_text
      .lines()
      .map(|line| line.chars().collect())
      .collect(),
  )
}

pub(crate) fn update_viewport(view_port: &mut ViewPort, player: &mut Player) {
  let left_boundary = view_port.x as f32 + view_port.width as f32 * 0.2;
  let right_boundary = view_port.x as f32 + view_port.width as f32 * 0.4;

  if player.x < left_boundary && view_port.x > 0 {
    view_port.x -= 1;
  }

  if player.x > right_boundary {
    view_port.x += 1;
  }
}
