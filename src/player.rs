use crate::map::{self, TileProperties};

pub(crate) const PLAYER_CHAR: char = 'V';

pub(crate) struct Player {
  pub(crate) x: f32,
  pub(crate) y: f32,
  pub(crate) vx: f32,
  pub(crate) vy: f32,
  pub(crate) on_ground: bool,
  pub(crate) climbing: bool,
  pub(crate) climb_cooldown: f32,
}

pub(crate) fn update_player_properties(player: &mut Player, map: &[Vec<map::Tile>], dt: f32) {
  let properties1: TileProperties = map[player.y as usize][player.x as usize].properties();
  let properties2: TileProperties = map[(player.y + 1.0) as usize][player.x as usize].properties();

  player.climbing = properties1.climbable;
  if player.climbing {
      player.vx = 0.0;
      player.vy = 0.0;
  }
  player.on_ground = properties2.standable;
  player.climb_cooldown = (player.climb_cooldown - dt).max(0.0);
}
