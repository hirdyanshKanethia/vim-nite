use crate::input;
use crate::map;
use crate::physics;
use crate::player;
use crate::renderer;

use std::thread;
use std::time::Duration;

pub(crate) fn game_loop() -> Result<(), Box<dyn std::error::Error>> {
  let map = map::load_map("./maps/map2.txt")?;

  let mut view_port = map::ViewPort { x: 0, width: 190, height: 43 };

  let mut player = player::Player {
    x: 5.0,
    y: map.len() as f32 - 5.0,
    vx: 0.0,
    vy: 0.0,
    on_ground: false,
  };

  let tick_rate = Duration::from_millis(16);

  let mut last_frame = std::time::Instant::now();

  loop {
    // let (term_width, term_height) = crossterm::terminal::size()?;

    // view_port.width = term_width as usize;
    // view_port.height = term_height as usize;

    let now = std::time::Instant::now();
    let dt = (now - last_frame).as_secs_f32();
    last_frame = now;

    if input::handle_input(&mut player, dt)? {
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
