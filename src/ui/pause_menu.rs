use ratatui::{
  Frame,
  layout::{Alignment, Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
  let area = f.area();

  let vertical = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage(40),
      Constraint::Length(7),
      Constraint::Percentage(40),
    ])
    .split(area);

  let horizontal = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(30),
      Constraint::Percentage(40),
      Constraint::Percentage(30),
    ])
    .split(vertical[1]);

  let popup_area = horizontal[1];

  f.render_widget(Clear, popup_area);

  let block = Block::default().title("Paused").borders(Borders::ALL);

  let options = ["Resume", "Main Menu"];

  let lines: Vec<Line> = options
    .iter()
    .enumerate()
    .map(|(i, text)| {
      if i == app.ui.selected_index {
        Line::from(Span::styled(
          format!("> {}", text),
          Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        ))
      } else {
        Line::from(format!("  {}", text))
      }
    })
    .collect();

  let paragraph = Paragraph::new(lines)
    .block(block)
    .alignment(Alignment::Center);

  f.render_widget(paragraph, popup_area);
}
