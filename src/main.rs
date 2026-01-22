fn main() -> Result<(), Box<dyn std::error::Error>> {
    vim_game::opening_prep()?;

    let result = vim_game::game_loop();

    vim_game::closing_prep()?;

    result
}
