use std::io::{Write, stdout};
use std::time::Duration;
use std::{char, fs, thread};

use crossterm::{
    QueueableCommand, cursor,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{self, Clear},
};

const PLAYER_CHAR: char = 'Λ';

struct ViewPort {
    x: usize,
    width: usize,
}

struct Player {
    x: usize,
    y: usize,
    vy: i32,
    on_ground: bool,
}

pub fn opening_prep() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;

    Ok(())
}

fn load_map(path: &str) -> std::io::Result<Vec<Vec<char>>> {
    let map_text = fs::read_to_string(path)?;

    Ok(map_text
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

fn render(
    map: &[Vec<char>],
    view_port_x: &usize,
    width: &usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    let (_, term_height) = crossterm::terminal::size()?;

    execute!(
        stdout,
        Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    let map_height = map.len();
    let start_y = term_height as i16 - map_height as i16;

    for (row_idx, row) in map.iter().enumerate() {
        let y = start_y + row_idx as i16;
        if y < 0 {
            continue;
        }

        stdout.queue(cursor::MoveTo(0, y as u16))?;

        let end_x = (*view_port_x + width).min(row.len());
        if let Some(slice) = row.get(*view_port_x..end_x) {
            for ch in slice {
                stdout.queue(Print(*ch))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}

fn render_player(map_buffer: &mut [Vec<char>], player: &Player) {
    if player.y < map_buffer.len() && player.x < map_buffer[player.y].len() {
        map_buffer[player.y][player.x] = PLAYER_CHAR;
    }
}

pub fn game_loop() -> Result<(), Box<dyn std::error::Error>> {
    let map = load_map("./maps/map1.txt")?;

    let (term_width, _) = crossterm::terminal::size()?;

    let mut view_port = ViewPort {
        x: 0,
        width: term_width as usize,
    };

    let mut player = Player {
        x: 5,
        y: 5,
        vy: 0,
        on_ground: false,
    };

    let tick_rate = Duration::from_millis(16);

    loop {
        let mut map_buffer = map.clone();

        render_player(&mut map_buffer, &player);

        render(&map_buffer, &view_port.x, &view_port.width)?;

        let frame_start = std::time::Instant::now();

        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // TODO: Add key handling functions rather than just inplace functions
                    KeyCode::Char('h') => view_port.x -= 1,
                    KeyCode::Char('l') => view_port.x += 1,
                    KeyCode::Char('j') => player.x += 1,
                    KeyCode::Char('k') => player.x -= 1,
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        let elapsed = frame_start.elapsed();
        if elapsed < tick_rate {
            thread::sleep(tick_rate - elapsed)
        }
    }

    Ok(())
}

pub fn closing_prep() -> Result<(), Box<dyn std::error::Error>> {
    terminal::disable_raw_mode()?;

    execute!(
        stdout(),
        Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    Ok(())
}
