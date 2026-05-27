use ratatui::{
  style::{Color, Style},
  text::{Line, Span},
};

use crate::game::{map, player};

// builds frames in lines from the tile arrays of map, returns these frames of lines to ratatui for
// rendering
pub(crate) fn build_frame_lines(
  map: &map::MapTiles,
  view_port: &map::ViewPort,
  player: &player::Player,
) -> Vec<Line<'static>> {
  let mut lines = Vec::new();

  let player_x = player.x.floor() as isize;
  let player_y = player.y.floor() as isize;

  let max_y = (view_port.y + view_port.height).min(map.tiles.len());

  let has_more_left = view_port.x > 0;
  let has_more_right = (view_port.x + view_port.width) < map.tiles[0].len();
  let has_more_up = view_port.y > 0;
  let has_more_down = (view_port.y + view_port.height) < map.tiles.len();

  for (world_y, row) in map.tiles.iter().enumerate().take(max_y).skip(view_port.y) {
    let max_x = (view_port.x + view_port.width).min(row.len());

    let mut spans = Vec::new();

    for (offset, tile) in row[view_port.x..max_x].iter().enumerate() {
      let world_x = offset + view_port.x;

      if world_x as isize == player_x && world_y as isize == player_y {
        let (char_to_draw, color) = if player.alive {
          (player::PLAYER_CHAR, Color::Green)
        } else {
          (player::DEAD_PLAYER_CHAR, Color::Red)
        };
        spans.push(Span::styled(
          char_to_draw.to_string(),
          Style::default().fg(color),
        ));
      } else {
        let is_top_edge = world_y == view_port.y;
        let is_bottom_edge = world_y == view_port.y + view_port.height - 1;
        let is_left_edge = offset == 0;
        let is_right_edge = offset == view_port.width - 1;

        let (char_to_draw, color) = if ((has_more_up && is_top_edge)
          || (has_more_down && is_bottom_edge))
          && *tile == map::Tile::Empty
        {
          ('┈', Color::DarkGray)
        } else if ((has_more_left && is_left_edge) || (has_more_right && is_right_edge))
          && *tile == map::Tile::Empty
        {
          ('┊', Color::DarkGray)
        } else {
          (tile.to_char(), tile.color())
        };

        spans.push(Span::styled(
          char_to_draw.to_string(),
          Style::default().fg(color),
        ));
      }
    }

    lines.push(Line::from(spans));
  }

  lines
}
