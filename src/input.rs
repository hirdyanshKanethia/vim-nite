use crate::{physics, player::Player};

use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub(crate) fn handle_input(
  // view_port: &mut ViewPort,
  player: &mut Player,
  dt: f32,
  // map: &[Vec<char>],
) -> Result<bool, Box<dyn std::error::Error>> {
  if !event::poll(Duration::from_millis(0))? {
    return Ok(false);
  }

  let Event::Key(key) = event::read()? else {
    return Ok(false);
  };

  Ok(match key.code {
    KeyCode::Char('h') => handle_move_left(player, dt),
    KeyCode::Char('l') => handle_move_right(player, dt),
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

fn handle_move_left(player: &mut Player, dt: f32) -> bool {
  if player.on_ground {
    player.vx -= physics::MOVE_ACCEL * dt;
    player.vx = player.vx.clamp(-physics::MAX_SPEED, physics::MAX_SPEED)
  }

  false
}

fn handle_move_right(player: &mut Player, dt: f32) -> bool {
  if player.on_ground {
    player.vx += physics::MOVE_ACCEL * dt;
    player.vx = player.vx.clamp(-physics::MAX_SPEED, physics::MAX_SPEED)
  }

  false
}

fn handle_jump(player: &mut Player) -> bool {
  if player.on_ground {
    player.vy = physics::JUMP_VELOCITY;
    player.on_ground = false;
  }

  false
}
