use crate::{map, player};

pub(crate) const GRAVITY: f32 = 30.0;
pub(crate) const JUMP_VELOCITY: f32 = -20.0;

pub(crate) const MOVE_ACCEL: f32 = 1000.0;
pub(crate) const MAX_SPEED: f32 = 30.0;
pub(crate) const FRICTION: f32 = 20.0;

fn is_solid(map: &[Vec<map::Tile>], x: usize, y: usize) -> bool {
  map
    .get(y)
    .and_then(|row| row.get(x))
    .map(|c| c.is_solid())
    .unwrap_or(true)
}

pub(crate) fn apply_physics(player: &mut player::Player, map: &[Vec<map::Tile>], dt: f32) {
  update_vertical_movement(player, map, dt);
  update_horizontal_movement(player, map, dt);
}

fn update_vertical_movement(player: &mut player::Player, map: &[Vec<map::Tile>], dt: f32) {
  let x = player.x.floor() as usize;

  let foot_y = (player.y + 1.0 + 0.01).floor() as usize;

  // set on_ground to false if player is not on a solid block
  if !is_solid(map, x, foot_y) {
    player.on_ground = false;
  }

  // apply gravity
  if !player.on_ground {
    player.vy += GRAVITY * dt;
  }

  let delta_y = player.vy * dt;
  if delta_y == 0.0 {
    return;
  }

  let x = player.x.floor() as usize;

  // player falling
  if delta_y > 0.0 {
    let start_y = (player.y + 1.0).floor() as i32;
    let end_y = (player.y + 1.0 + delta_y).floor() as i32;

    for tile_y in start_y..=end_y {
      // if solid block found while falling
      if is_solid(map, x, tile_y as usize) {
        player.vy = 0.0;
        player.y = tile_y as f32 - 1.0;
        player.on_ground = true;
        return;
      }
    }

    // no collision
    player.y += delta_y;
    player.on_ground = false;
  }
  // jumping upwards
  else {
    let start_y = player.y.floor() as i32;
    let end_y = (player.y + delta_y) as i32;

    // collision
    for tile_y in (end_y..=start_y).rev() {
      if is_solid(map, x, tile_y as usize) {
        player.y = tile_y as f32 + 1.0;
        player.vy = 0.0;
        return;
      }
    }

    player.y += delta_y;
  }
}

fn update_horizontal_movement(player: &mut player::Player, map: &[Vec<map::Tile>], dt: f32) {
  // apply friction
  if player.on_ground {
    let friction = FRICTION * dt;

    if player.vx > 0.0 {
      player.vx = (player.vx - friction).max(0.0);
    } else {
      player.vx = (player.vx + friction).min(0.0);
    }
  }

  let delta_x = player.vx * dt;
  if delta_x == 0.0 {
    return;
  }

  let y = player.y.floor() as usize;

  // going rhs
  if delta_x > 0.0 {
    let start_x = (player.x + 1.0).floor() as i32;
    let end_x = (player.x + 1.0 + delta_x).floor() as i32;

    for tile_x in start_x..=end_x {
      // collision
      if is_solid(map, tile_x as usize, y) {
        player.x = tile_x as f32 - 1.0;
        player.vx = 0.0;
        return;
      }
    }

    // no collision
    player.x += delta_x;
  }
  // going lhs
  else {
    let start_x = player.x.floor() as i32;
    let end_x = (player.x + delta_x).floor() as i32;

    for tile_x in (end_x..=start_x).rev() {
      // collision
      if is_solid(map, tile_x as usize, y) {
        player.x = tile_x as f32 + 1.0;
        player.vx = 0.0;
        return;
      }
    }

    // no collision
    player.x += delta_x;
  }

  if player.vx < 1.0 && player.vx > -1.0 {
    player.vx = 0.0;
  }
}
