use crate::physics;
use crate::map;
use crate::player;
use crate::renderer;

use crossterm::event::{self, Event, KeyCode};
use std::thread;
use std::time::Duration;

pub(crate) fn game_loop() -> Result<(), Box<dyn std::error::Error>> {
    let map = map::load_map("./maps/map1.txt")?;

    let (term_width, _) = crossterm::terminal::size()?;

    let mut view_port = map::ViewPort {
        x: 0,
        width: term_width as usize,
    };

    let mut player = player::Player {
        x: 5.0,
        y: 20.0,
        vy: 0.0,
        on_ground: false,
    };

    let tick_rate = Duration::from_millis(16);

    let mut last_frame = std::time::Instant::now();

    loop {
        let now = std::time::Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        last_frame = now;

        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // TODO: Add key handling functions rather than just inplace functions
                    KeyCode::Char('h') => view_port.x -= 1,
                    KeyCode::Char('l') => view_port.x += 1,
                    KeyCode::Char('j') => player.x += 1.0,
                    KeyCode::Char('k') => player.x -= 1.0,
                    KeyCode::Char('n') => {
                        if player.on_ground {
                            player.vy = physics::JUMP_VELOCITY;
                            player.on_ground = false;
                        }
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        physics::update_physics(&mut player, &map, dt);

        let mut map_buffer = map.clone();
        player::update_player(&mut map_buffer, &player);
        renderer::render(&map_buffer, view_port.x, view_port.width)?;

        let frame_time = now.elapsed();
        if frame_time < tick_rate {
            thread::sleep(tick_rate - frame_time);
        }
    }

    Ok(())
}
