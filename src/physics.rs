use crate::player;

pub(crate) const GRAVITY: f32 = 30.0;
pub(crate) const JUMP_VELOCITY: f32 = -20.0;

fn is_solid(map: &[Vec<char>], x: usize, y: usize) -> bool {
  map
    .get(y)
    .and_then(|row| row.get(x))
    .map(|c| *c != ' ')
    .unwrap_or(true)
}

pub(crate) fn update_physics(player: &mut player::Player, map: &[Vec<char>], dt: f32) {
  // apply gravity
  if !player.on_ground {
    player.vy += GRAVITY * dt;
  }

  let next_y = player.y + player.vy * dt;

  // collision check
  if player.vy > 0.0 {
    // falling
    let foot_y = (next_y + 1.0).floor() as usize;
    let x = player.x.floor() as usize;

    if is_solid(map, x, foot_y) {
      player.y = foot_y as f32 - 1.0;
      player.vy = 0.0;
      player.on_ground = true;
    } else {
      player.y = next_y;
      player.on_ground = false;
    }
  } else {
    // jumping upward
    let head_y = next_y.floor() as usize;
    let x = player.x.floor() as usize;

    if is_solid(map, x, head_y) {
      player.vy = 0.0;
    } else {
      player.y = next_y;
    }
  }
}
