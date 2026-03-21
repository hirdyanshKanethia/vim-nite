use crate::{
  app::AppState,
  game::{map::Tile, physics, player::Player},
};

use crossterm::event::{KeyCode, KeyEvent};

// ------------------------------
// Functions to handle game input
// ------------------------------

pub(crate) fn handle_input(
  player: &mut Player,
  key: KeyEvent,
  dt: f32,
  map: &[Vec<Tile>],
) -> AppState {
  match key.code {
    KeyCode::Char('h') => handle_move_left(player, map, dt),
    KeyCode::Char('l') => handle_move_right(player, map, dt),
    KeyCode::Char('k') => handle_up(player, map),
    KeyCode::Char('j') => handle_down(player, map),
    KeyCode::Char(';') | KeyCode::Char(':') => AppState::EnteringCommand,
    KeyCode::Char('q') => AppState::Paused,
    _ => AppState::Playing,
  }
}

fn handle_move_left(player: &mut Player, map: &[Vec<Tile>], dt: f32) -> AppState {
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
    player.climb_cooldown = physics::CLIMB_COOLDOWN;
    player.x -= 1.0;
  }

  AppState::Playing
}

fn handle_move_right(player: &mut Player, map: &[Vec<Tile>], dt: f32) -> AppState {
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
    player.climb_cooldown = physics::CLIMB_COOLDOWN;
    player.x += 1.0;
  }

  AppState::Playing
}

fn handle_up(player: &mut Player, map: &[Vec<Tile>]) -> AppState {
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
    player.climb_cooldown = physics::CLIMB_COOLDOWN;
    player.y -= 1.0;
  }

  AppState::Playing
}

fn handle_down(player: &mut Player, map: &[Vec<Tile>]) -> AppState {
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
    player.climb_cooldown = physics::CLIMB_COOLDOWN;
    player.y += 1.0;
  }

  AppState::Playing
}
