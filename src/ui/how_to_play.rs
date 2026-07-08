use ratatui::{
  Frame,
  layout::{Alignment, Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

struct Slide<'a> {
  title: &'a str,
  visual: Vec<Line<'a>>,
  instructions: Vec<Line<'a>>,
}

pub fn render(f: &mut Frame, app: &App) {
  let size = f.area();

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Min(2),
      Constraint::Length(3),
      Constraint::Length(6),
      Constraint::Length(1),
      Constraint::Length(8),
      Constraint::Length(3),
      Constraint::Min(2),
    ])
    .split(size);

  let slides = [
    Slide {
      title: "--- How to Play: The Basics ---",
      visual: vec![
        Line::from(""), // Vertical centering
        Line::from(vec![Span::styled(
          "        𝑽        ",
          Style::default().fg(Color::Green),
        )]),
        Line::from(vec![Span::styled(
          "      █████      ",
          Style::default().fg(Color::DarkGray),
        )]),
        Line::from(""),
      ],
      instructions: vec![
        Line::from("Welcome to vim-nite! Your goal is to navigate the levels."),
        Line::from(""),
        Line::from("Movement:"),
        Line::from("  h : Move Left"),
        Line::from("  l : Move Right"),
        Line::from("  k : Jump / Climb Up"),
        Line::from("  j : Climb Down"),
      ],
    },
    Slide {
      title: "--- Environment Elements ---",
      visual: vec![
        Line::from(vec![
          Span::raw("  "),
          Span::styled("#", Style::default().fg(Color::Yellow)),
          Span::styled("█", Style::default().fg(Color::DarkGray)),
          Span::raw("             "), // 13 spaces
        ]),
        Line::from(vec![
          Span::raw("  "),
          Span::styled("#", Style::default().fg(Color::Yellow)),
          Span::styled("█", Style::default().fg(Color::DarkGray)),
          Span::raw("             "), // 13 spaces
        ]),
        Line::from(vec![
          Span::raw("  "),
          Span::styled("#", Style::default().fg(Color::Yellow)),
          Span::styled("█", Style::default().fg(Color::DarkGray)),
          Span::raw("   "),
          Span::styled("▲▲▲", Style::default().fg(Color::LightRed)),
          Span::raw("   "),
          Span::styled("§", Style::default().fg(Color::Blue)),
          Span::raw("   "), // 3 spaces
        ]),
        Line::from(vec![Span::styled(
          "█████████████████",
          Style::default().fg(Color::DarkGray),
        )]),
      ],
      instructions: vec![
        Line::from(vec![
          Span::raw("  "),
          Span::styled("█", Style::default().fg(Color::DarkGray)),
          Span::raw(" : Wall. Solid ground you can stand on."),
        ]),
        Line::from(vec![
          Span::raw("  "),
          Span::styled("#", Style::default().fg(Color::Yellow)),
          Span::raw(" : Ladder. Use 'k' and 'j' to climb up and down."),
        ]),
        Line::from(vec![
          Span::raw("  "),
          Span::styled("▲", Style::default().fg(Color::LightRed)),
          Span::raw(" : Spike. Deadly! Avoid these."),
        ]),
        Line::from(vec![
          Span::raw("  "),
          Span::styled("§", Style::default().fg(Color::Blue)),
          Span::raw(" : Checkpoint. If you die, you respawn here."),
        ]),
      ],
    },
    Slide {
      title: "--- Objectives ---",
      visual: vec![
        Line::from(vec![
          Span::raw(" "),
          Span::styled("@", Style::default().fg(Color::LightBlue)),
          Span::raw("         "),
          Span::styled("X", Style::default().fg(Color::Cyan)),
          Span::raw(" "),
        ]),
        Line::from(vec![
          Span::raw(" "),
          Span::styled("█", Style::default().fg(Color::DarkGray)),
          Span::raw("         "),
          Span::styled("█", Style::default().fg(Color::DarkGray)),
          Span::raw(" "),
        ]),
        Line::from(vec![
          Span::raw(" "),
          Span::styled("█", Style::default().fg(Color::DarkGray)),
          Span::raw("  "),
          Span::styled("█████", Style::default().fg(Color::DarkGray)),
          Span::raw("  "),
          Span::styled("█", Style::default().fg(Color::DarkGray)),
          Span::raw(" "),
        ]),
        Line::from(vec![
          Span::raw(" "),
          Span::styled("███████████", Style::default().fg(Color::DarkGray)),
          Span::raw(" "),
        ]),
      ],
      instructions: vec![
        Line::from(vec![
          Span::raw("  "),
          Span::styled("@", Style::default().fg(Color::LightBlue)),
          Span::raw(" : Player Start. Where you begin the map."),
        ]),
        Line::from(vec![
          Span::raw("  "),
          Span::styled("X", Style::default().fg(Color::Cyan)),
          Span::raw(" : Map Exit. Reach this tile to complete the level."),
        ]),
        Line::from(""),
        Line::from("Reach the exit safely to win!"),
      ],
    },
  ];

  let current_slide_idx = app.ui.slideshow_index;
  if current_slide_idx >= slides.len() {
    return; // Safety guard
  }

  let slide = &slides[current_slide_idx];

  // Title
  let title = Paragraph::new(slide.title)
    .alignment(Alignment::Center)
    .style(
      Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD),
    );
  f.render_widget(title, chunks[1]);

  // Visual Example (smaller window in the center)
  let visual_h_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(30),
      Constraint::Percentage(40),
      Constraint::Percentage(30),
    ])
    .split(chunks[2]);

  let visual = Paragraph::new(slide.visual.clone())
    .alignment(Alignment::Center)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(77, 124, 15))),
    );
  f.render_widget(visual, visual_h_chunks[1]);

  // Instructions
  let instructions = Paragraph::new(slide.instructions.clone()).alignment(Alignment::Center);
  f.render_widget(instructions, chunks[4]);

  // Footer
  let footer = Paragraph::new(format!(
    "Slide {}/{} | 'l' for Next | 'h' for Previous | 'q' to Quit",
    current_slide_idx + 1,
    slides.len()
  ))
  .alignment(Alignment::Center)
  .style(Style::default().fg(Color::DarkGray));

  f.render_widget(footer, chunks[5]);
}
