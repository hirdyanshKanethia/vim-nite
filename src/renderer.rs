use crossterm::{
  QueueableCommand, cursor, execute,
  style::Print,
  terminal::{self, Clear},
};
use std::io::{Write, stdout};

use crate::{map, player};

pub(crate) fn opening_prep() -> Result<(), Box<dyn std::error::Error>> {
  terminal::enable_raw_mode()?;

  Ok(())
}

pub(crate) fn render(
  map: &[Vec<map::Tile>],
  view_port_x: usize,
  width: usize,
  player: &player::Player,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut stdout = stdout();
  let (_, term_height) = crossterm::terminal::size()?;

  execute!(
    stdout,
    Clear(terminal::ClearType::All),
    cursor::MoveTo(0, 0)
  )?;

  let map_height = map.len();
  let start_y = term_height as i16 - map_height as i16;

  let player_x = player.x.floor() as isize;
  let player_y = player.y.floor() as isize;

  for (row_idx, row) in map.iter().enumerate() {
    let y = start_y + row_idx as i16;
    if y < 0 {
      continue;
    }

    stdout.queue(cursor::MoveTo(0, y as u16))?;

    let end_x = (view_port_x + width).min(row.len());

    for (col_idx, tile) in row[view_port_x..end_x].iter().enumerate() {
      let world_x = view_port_x as isize + col_idx as isize;
      let world_y = row_idx as isize;

      if world_x == player_x && world_y == player_y {
        stdout.queue(Print(player::PLAYER_CHAR))?;
      } else {
        stdout.queue(Print(tile.to_char()))?;
      }
    }
  }

  stdout.flush()?;
  Ok(())
}

pub(crate) fn closing_prep() -> Result<(), Box<dyn std::error::Error>> {
  terminal::disable_raw_mode()?;

  execute!(
    stdout(),
    Clear(terminal::ClearType::All),
    cursor::MoveTo(0, 0)
  )?;

  Ok(())
}
