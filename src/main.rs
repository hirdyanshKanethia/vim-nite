mod game;
mod input;
mod map;
mod physics;
mod player;
mod renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  renderer::opening_prep()?;

  let result = game::game_loop();

  renderer::closing_prep()?;

  result
}
