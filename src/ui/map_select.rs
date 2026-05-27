use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
  let size = f.area();

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
    .map(|info| {
      // 1. Determine base color
      let name_color = if info.is_valid { Color::Green } else { Color::Red };

      // 2. Fetch stats for this map
      let stats_text = if info.completions > 0 {
        format!(
          "  [ Best: {:.3}s | {} Completions ]",
          info.best_time_ms.unwrap_or(0) as f64 / 1000.0,
          info.completions
        )
      } else {
        "  [ Unbeaten ]".to_string()
      };

      // 3. Build a styled Line
      let line = Line::from(vec![
        Span::styled(
          info.name.clone(),
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
