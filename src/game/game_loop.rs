use crate::game::input;
use crate::game::map;
use crate::game::physics;
use crate::game::player;
use crate::game::player::update_player_properties;
use crate::game::renderer;

use std::thread;
use std::time::Duration;

// TODO: 1) Add ratatui to the system to get a bottom hotbar
//       2) Add block types like spikes and ladders - ladders done
//       3) Add start and finish points, maybe checkpoints too

pub(crate) fn game_loop() -> Result<(), Box<dyn std::error::Error>> {
  let map = map::load_map("./maps/map2.txt")?;

  let mut view_port = map::ViewPort {
    x: 0,
    width: 190,
    height: 43,
  };

  let mut player = player::Player {
    x: 5.0,
    y: map.len() as f32 - 5.0,
    vx: 0.0,
    vy: 0.0,
    on_ground: false,
    climbing: false,
    climb_cooldown: 0.0,
    lives: 3,
  };

  let tick_rate = Duration::from_millis(16);

  let mut last_frame = std::time::Instant::now();

  loop {
    let now = std::time::Instant::now();
    let dt = (now - last_frame).as_secs_f32();
    last_frame = now;

    update_player_properties(&mut player, &map, dt);

    if input::handle_input(&mut player, dt, &map)? {
      break;
    }

    physics::apply_physics(&mut player, &map, dt);

    map::update_viewport(&mut view_port, &player);

    renderer::render(&map, &view_port, &player)?;

    let frame_time = now.elapsed();
    if frame_time < tick_rate {
      thread::sleep(tick_rate - frame_time);
    }
  }

  Ok(())
}
