use ratatui::{
  Frame,
  layout::Alignment,
  layout::Rect,
  style::{Color, Modifier, Style},
  text::Line,
  widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, area: Rect) {
  let block = Block::default()
    .title("Checkpoint Reached")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Green));

  let content = vec![
    Line::from(""),
    Line::from("Press Enter to Continue"),
    Line::from(""),
  ];

  let paragraph = Paragraph::new(content)
    .block(block)
    .alignment(Alignment::Center)
    .style(
      Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD),
    );

  f.render_widget(paragraph, area);
}
