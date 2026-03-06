use ratatui::{
  Frame,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  text::Line,
  widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::{App, AppState, MessageType};

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
  let vertical = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

  let horizontal = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical[1]);

  horizontal[1]
}

pub fn render(f: &mut Frame, app: &App) {
  let area = centered_rect(50, 25, f.area());

  // Clear background under popup
  f.render_widget(Clear, area);

  let message_type = match app.state {
    AppState::Message(m) => m,
    _ => return,
  };

  let (title, text, color) = match message_type {
    MessageType::Death => ("You Died", "Press Enter to Respawn", Color::Red),
    MessageType::Checkpoint => (
      "Checkpoint Reached",
      "Press Enter to Continue",
      Color::Green,
    ),
  };

  let block = Block::default()
    .title(title)
    .borders(Borders::ALL)
    .border_style(Style::default().fg(color));

  let content = vec![Line::from(""), Line::from(text), Line::from("")];

  let paragraph = Paragraph::new(content)
    .block(block)
    .alignment(Alignment::Center)
    .style(Style::default().fg(color).add_modifier(Modifier::BOLD));

  f.render_widget(paragraph, area);
}
