use crate::game::map::{self, TileProperties};

pub(crate) const PLAYER_CHAR: char = 'V';
pub(crate) const DEAD_PLAYER_CHAR: char = '†';

// Player struct and methods
// TODO: Add here the implemenation of health decrement upon dead and respawn thing

pub(crate) struct Player {
  pub(crate) x: f32,
  pub(crate) y: f32,
  pub(crate) vx: f32,
  pub(crate) vy: f32,
  pub(crate) on_ground: bool,
  pub(crate) climbing: bool,
  pub(crate) climb_cooldown: f32,
  pub(crate) lives: i16,
  pub(crate) alive: bool,
  pub(crate) respawn: (f32, f32),
}

pub(crate) fn update_player_properties(player: &mut Player, map: &[Vec<map::Tile>], dt: f32) {
  let player_block_props: TileProperties = map[player.y as usize][player.x as usize].properties();
  let top_block_props: TileProperties =
    map[(player.y - 1.0) as usize][player.x as usize].properties();
  let below_block_props: TileProperties =
    map[(player.y + 1.0) as usize][player.x as usize].properties();
  let left_block_props: TileProperties =
    map[(player.y) as usize][(player.x - 1.0) as usize].properties();
  let right_block_props: TileProperties =
    map[(player.y) as usize][(player.x + 1.0) as usize].properties();

  player.climbing = player_block_props.climbable;
  if player.climbing {
    player.vx = 0.0;
    player.vy = 0.0;
  }
  player.on_ground = below_block_props.standable;
  player.climb_cooldown = (player.climb_cooldown - dt).max(0.0);

  // If player is standing in a deadly block, raise interrupt and deduct life
  if player_block_props.deadly {
    player.alive = false;
    player.lives -= 1;
  }
}
