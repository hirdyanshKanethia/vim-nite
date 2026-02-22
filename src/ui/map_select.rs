use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout},
  style::{Modifier, Style},
  widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
  let size = f.area();

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage(20),
      Constraint::Percentage(60),
      Constraint::Percentage(20),
    ])
    .split(size);

  let items: Vec<ListItem> = app
    .available_maps
    .iter()
    .map(|name| ListItem::new(name.clone()))
    .collect();

  let list = List::new(items)
    .block(Block::default().title("Select Map").borders(Borders::ALL))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

  let mut state = ratatui::widgets::ListState::default();
  state.select(Some(app.ui.selected_index));

  f.render_stateful_widget(list, chunks[1], &mut state);
}
