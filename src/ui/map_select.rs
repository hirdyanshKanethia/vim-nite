use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, game::map::is_map_valid, game::save::SaveData};

pub fn render(f: &mut Frame, app: &App) {
  let size = f.area();
  // Load stats once at the start of the frame to avoid repeated disk I/O
  let save_data = SaveData::load();

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage(10), // Reduced padding for more list space
      Constraint::Min(1),
      Constraint::Percentage(10),
    ])
    .split(size);

  let items: Vec<ListItem> = app
    .available_maps
    .iter()
    .map(|name| {
      let path = format!("./maps/{}", name);
      let is_valid = is_map_valid(&path);

      // 1. Determine base color
      let name_color = if is_valid { Color::Green } else { Color::Red };

      // 2. Fetch stats for this map
      let stats_text = if let Some(stats) = save_data.maps.get(name) {
        if stats.completions > 0 {
          format!(
            "  [ Best: {:.3}s | {} Completions ]",
            stats.best_time_ms as f64 / 1000.0,
            stats.completions
          )
        } else {
          "  [ Unbeaten ]".to_string()
        }
      } else {
        "  [ New Map ]".to_string()
      };

      // 3. Build a styled Line
      let line = Line::from(vec![
        Span::styled(
          name.clone(),
          Style::default().fg(name_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
          stats_text,
          Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
        ),
      ]);

      ListItem::new(line)
    })
    .collect();

  let list = List::new(items)
    .block(
      Block::default()
        .title(" SELECT MAP ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
    )
    .highlight_style(
      Style::default()
        .bg(Color::Indexed(237))
        .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol("❯ "); // Cleaner Vim-like symbol

  let mut state = ratatui::widgets::ListState::default();
  state.select(Some(app.ui.selected_index));

  f.render_stateful_widget(list, chunks[1], &mut state);
}
