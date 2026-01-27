use std::fs;

pub(crate) struct ViewPort {
  pub(crate) x: usize,
  pub(crate) width: usize,
}

pub(crate) fn load_map(path: &str) -> std::io::Result<Vec<Vec<char>>> {
  let map_text = fs::read_to_string(path)?;

  Ok(
    map_text
      .lines()
      .map(|line| line.chars().collect())
      .collect(),
  )
}
