use ratatui::{
  Frame,
  layout::{Alignment, Rect},
  style::{Color, Modifier, Style},
  text::Line,
  widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, area: Rect) {
  let block = Block::default()
    .title("Game Over")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Red));

  let content = vec![
    Line::from(""),
    Line::from("You lost all your lives."),
    Line::from("Press Enter to return to the main menu."),
    Line::from(""),
  ];

  let paragraph = Paragraph::new(content)
    .block(block)
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

  f.render_widget(paragraph, area);
}
