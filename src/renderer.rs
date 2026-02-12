use crossterm::{
  QueueableCommand, cursor, execute,
  style::Print,
  terminal::{self, Clear, EnterAlternateScreen},
};
use std::io::{Write, stdout};

use crate::{map, player};

pub(crate) fn opening_prep() -> Result<(), Box<dyn std::error::Error>> {
  terminal::enable_raw_mode()?;
  execute!(stdout(), EnterAlternateScreen)?;

  Ok(())
}

pub(crate) fn render(
  map: &[Vec<map::Tile>],
  view_port: &map::ViewPort,
  player: &player::Player,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut stdout = stdout();

  execute!(
    stdout,
    Clear(terminal::ClearType::All),
    cursor::MoveTo(0, 0)
  )?;

  let player_x = player.x.floor() as isize;
  let player_y = player.y.floor() as isize;

  let max_y = (view_port.height + view_port.height).min(map.len());

  for (screen_y, world_y) in (view_port.height..max_y).enumerate() {
    let row = &map[world_y];

    stdout.queue(cursor::MoveTo(0, screen_y as u16))?;

    let max_x = (view_port.x + view_port.width).min(row.len());

    for (offset, tile) in row[view_port.x..max_x].iter().enumerate() {
      let world_x = offset + view_port.x;

      if world_x as isize == player_x && world_y as isize == player_y {
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
