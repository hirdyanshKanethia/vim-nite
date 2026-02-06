use crate::{map::ViewPort, physics, player::Player};

use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub(crate) fn handle_input(
  view_port: &mut ViewPort,
  player: &mut Player,
) -> Result<bool, Box<dyn std::error::Error>> {
  if !event::poll(Duration::from_millis(0))? {
    return Ok(false);
  }

  let Event::Key(key) = event::read()? else {
    return Ok(false);
  };

  Ok(match key.code {
    // TODO: Add key handling functions rather than just inplace functions
    KeyCode::Char('h') => handle_move_left(view_port, player),
    KeyCode::Char('l') => handle_move_right(view_port, player),
    KeyCode::Char('k') => handle_jump(player),
    // KeyCode::Char('j') => player.x += 1.0,
    // KeyCode::Char('k') => {
    //   if player.on_ground {
    //     player.vy = physics::JUMP_VELOCITY;
    //     player.on_ground = false;
    //   }
    // }
    KeyCode::Char('q') => true,
    _ => false,
  })
}

pub(crate) fn handle_move_left(view_port: &mut ViewPort, player: &mut Player) -> bool {
  let width = view_port.width as f32;
  let x = view_port.x as f32;

  // If view port is at left limit
  if view_port.x == 0 {
    player.x -= 1.0;
  } else {
    player.x -= 1.0;

    // If player is at the edge of the imaginary left boundary
    if (width / 10.0) + x >= player.x {
      view_port.x -= 1;
    }
  }

  false
}

pub(crate) fn handle_move_right(view_port: &mut ViewPort, player: &mut Player) -> bool {
  let width = view_port.width as f32;
  let x = view_port.x as f32;

  player.x += 1.0;

  // If player is at the edge of the imaginary right boundary
  if (width / 3.0) + x <= player.x {
    view_port.x += 1;
  }

  false
}

pub(crate) fn handle_jump(player: &mut Player) -> bool {
  if player.on_ground {
    player.vy = physics::JUMP_VELOCITY;
    player.on_ground = false;
  }

  false
}
