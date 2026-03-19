use ratatui::{
  Frame,
  layout::{Alignment, Rect},
  style::{Color, Modifier, Style},
  text::Line,
  widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, area: Rect) {
  let block = Block::default()
    .title("You Win!")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Yellow));

  let content = vec![
    Line::from(""),
    Line::from("🎉 Congratulations! 🎉"),
    Line::from(""),
    Line::from("You reached the end of the map"),
    Line::from(""),
    Line::from("Press Enter to Continue"),
    Line::from(""),
  ];

  let paragraph = Paragraph::new(content)
    .block(block)
    .alignment(Alignment::Center)
    .style(
      Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD),
    );

  f.render_widget(paragraph, area);
}
