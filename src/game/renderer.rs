use ratatui::text::Line;

use crate::game::{map, player};

// builds frames in lines from the tile arrays of map, returns these frames of lines to ratatui for
// rendering
pub(crate) fn build_frame_lines(
  map: &[Vec<map::Tile>],
  view_port: &map::ViewPort,
  player: &player::Player,
) -> Vec<Line<'static>> {
  let mut lines = Vec::new();

  let player_x = player.x.floor() as isize;
  let player_y = player.y.floor() as isize;

  let max_y = (view_port.y + view_port.height).min(map.len());

  for (world_y, row) in map.iter().enumerate().take(max_y).skip(view_port.y) {
    let max_x = (view_port.x + view_port.width).min(row.len());

    let mut line = String::new();

    for (offset, tile) in row[view_port.x..max_x].iter().enumerate() {
      let world_x = offset + view_port.x;

      if world_x as isize == player_x && world_y as isize == player_y {
        if player.alive {
          line.push(player::PLAYER_CHAR);
        } else {
          line.push(player::DEAD_PLAYER_CHAR);
        }
      } else {
        line.push(tile.to_char());
      }
    }

    lines.push(Line::from(line));
  }

  lines
}
