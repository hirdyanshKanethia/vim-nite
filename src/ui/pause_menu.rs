use ratatui::{
  Frame,
  layout::{Alignment, Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
  let area = f.area();

  let vertical = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage(35),
      Constraint::Length(10),
      Constraint::Percentage(35),
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

  let map_name = if let Some(game) = &app.game {
      game.map_name.as_str()
  } else {
      "Unknown"
  };

  let block = Block::default()
    .title(format!(" PAUSED: {} ", map_name))
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(Style::default().fg(Color::Rgb(57, 211, 83)));

  let options = ["Resume", "Main Menu"];

  let mut lines = Vec::new();
  lines.push(Line::from(""));
  lines.push(Line::from(Span::styled("Take a breather.", Style::default().fg(Color::DarkGray))));
  lines.push(Line::from(""));

  for (i, text) in options.iter().enumerate() {
      if i == app.ui.selected_index {
        lines.push(Line::from(Span::styled(
          format!(">> {} ", text),
          Style::default()
            .fg(Color::Black)
            .bg(Color::Rgb(57, 211, 83))
            .add_modifier(Modifier::BOLD),
        )));
      } else {
        lines.push(Line::from(format!("   {} ", text)));
      }
  }

  let paragraph = Paragraph::new(lines)
    .block(block)
    .alignment(Alignment::Center);

  f.render_widget(paragraph, popup_area);
}
