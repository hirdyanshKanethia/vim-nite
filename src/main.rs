mod game;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  game::renderer::opening_prep()?;

  let result = game::game_loop::game_loop();

  game::renderer::closing_prep()?;

  result
}
