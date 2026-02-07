const PLAYER_CHAR: char = 'Λ';

pub(crate) struct Player {
  pub(crate) x: f32,
  pub(crate) y: f32,
  pub(crate) vx: f32,
  pub(crate) vy: f32,
  pub(crate) on_ground: bool,
}

pub(crate) fn update_player(map: &mut [Vec<char>], player: &Player) {
  let x = player.x.floor() as isize;
  let y = player.y.floor() as isize;

  if y >= 0 && y < map.len() as isize {
    let row = &mut map[y as usize];
    if x >= 0 && x < row.len() as isize {
      row[x as usize] = PLAYER_CHAR;
    }
  }
}
