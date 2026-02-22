use crate::game::{
  map::{self, Tile},
  physics,
  player::Player,
};

use crossterm::event::{KeyCode, KeyEvent};

pub(crate) fn handle_input(
  player: &mut Player,
  key: KeyEvent,
  dt: f32,
  map: &[Vec<Tile>],
) -> Result<bool, Box<dyn std::error::Error>> {
  Ok(match key.code {
    KeyCode::Char('h') => handle_move_left(player, map, dt),
    KeyCode::Char('l') => handle_move_right(player, map, dt),
    KeyCode::Char('k') => handle_up(player, map),
    KeyCode::Char('j') => handle_down(player, map),
    KeyCode::Char('q') => true,
    _ => false,
  })
}

fn handle_move_left(player: &mut Player, map: &[Vec<Tile>], dt: f32) -> bool {
  if player.on_ground {
    player.vx -= physics::MOVE_ACCEL * dt;
    player.vx = player.vx.clamp(-physics::MAX_SPEED, physics::MAX_SPEED)
  }

  if !map[player.y as usize][(player.x - 1.0) as usize]
    .properties()
    .solid
    && player.climbing
    && player.climb_cooldown == 0.0
  {
    player.climb_cooldown = map::CLIMB_COOLDOWN;
    player.x -= 1.0;
  }

  false
}

fn handle_move_right(player: &mut Player, map: &[Vec<Tile>], dt: f32) -> bool {
  if player.on_ground {
    player.vx += physics::MOVE_ACCEL * dt;
    player.vx = player.vx.clamp(-physics::MAX_SPEED, physics::MAX_SPEED)
  }

  if !map[player.y as usize][(player.x + 1.0) as usize]
    .properties()
    .solid
    && player.climbing
    && player.climb_cooldown == 0.0
  {
    player.climb_cooldown = map::CLIMB_COOLDOWN;
    player.x += 1.0;
  }

  false
}

fn handle_up(player: &mut Player, map: &[Vec<Tile>]) -> bool {
  // Case for when player is jumping
  if !player.climbing && player.on_ground {
    player.vy = physics::JUMP_VELOCITY;
    player.on_ground = false;
  }
  // Case for when player is climbing
  if player.climbing
    && !map[(player.y - 1.0) as usize][player.x as usize]
      .properties()
      .solid
    && player.climb_cooldown == 0.0
  {
    player.climb_cooldown = map::CLIMB_COOLDOWN;
    player.y -= 1.0;
  }

  false
}

fn handle_down(player: &mut Player, map: &[Vec<Tile>]) -> bool {
  // Simply climb down while current position is of climbing
  if (player.climbing
    && !map[(player.y + 1.0) as usize][player.x as usize]
      .properties()
      .solid
    && player.climb_cooldown == 0.0)
     // Climb down if current position is not of climbing but the block below is climbable
    || (map[(player.y + 1.0) as usize][player.x as usize]
      .properties()
      .climbable && player.climb_cooldown == 0.0)
  {
    player.climb_cooldown = map::CLIMB_COOLDOWN;
    player.y += 1.0;
  }

  false
}
