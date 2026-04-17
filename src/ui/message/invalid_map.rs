use ratatui::{
  Frame,
  layout::Alignment,
  layout::Rect,
  style::{Color, Modifier, Style},
  text::Line,
  widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render(f: &mut Frame, area: Rect) {
  let block = Block::default()
    .title(" INVALID MAP ")
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Thick) // Matches your "heavy" theme
    .border_style(Style::default().fg(Color::Red));

  let content = vec![
    Line::from(""),
    Line::from("Map requirements not met:"),
    Line::from("Map must contain '@' (Start) and 'X' (Exit)"),
    Line::from(""),
    Line::from("Check guidelines in maps/README.txt"),
    Line::from(""),
    Line::from("Press <Enter> to Return"),
  ];

  let paragraph = Paragraph::new(content)
    .block(block)
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

  f.render_widget(paragraph, area);
}
