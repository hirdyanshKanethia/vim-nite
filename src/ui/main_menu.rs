use crate::app::App;
use ratatui::{
  Frame,
  layout::{Alignment, Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph},
};

fn is_block_char(c: char) -> bool {
  matches!(c, '█' | '▀' | '▄' | '▌' | '▐' | '▝' | '▘' | '▗' | '▖')
}

fn parse_logo(content: &str) -> Vec<Line<'static>> {
  let v_style = Style::default()
    .fg(Color::Rgb(57, 211, 83))
    .add_modifier(Modifier::BOLD);

  let im_style = Style::default()
    .fg(Color::Rgb(200, 200, 200))
    .add_modifier(Modifier::BOLD);

  content
    .lines()
    .map(|line| {
      let mut spans: Vec<Span<'static>> = Vec::new();
      let mut current = String::new();
      let mut current_is_block = false;

      for c in line.chars() {
        let char_is_block = is_block_char(c);

        if current.is_empty() {
          current_is_block = char_is_block;
          current.push(c);
        } else if char_is_block == current_is_block {
          current.push(c);
        } else {
          let style = if current_is_block { im_style } else { v_style };
          spans.push(Span::styled(current.clone(), style));
          current.clear();
          current_is_block = char_is_block;
          current.push(c);
        }
      }

      if !current.is_empty() {
        let style = if current_is_block { im_style } else { v_style };
        spans.push(Span::styled(current, style));
      }

      Line::from(spans)
    })
    .collect()
}

pub fn render(f: &mut Frame, app: &App) {
  let size = f.area();

  // Global Wrapper
  let global_block = Block::default()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(Style::default().fg(Color::Rgb(57, 211, 83)));

  let inner_area = global_block.inner(size);
  f.render_widget(global_block, size);

  let h_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
    .split(inner_area);

  // Left Column: Logo + Menu
  let left_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Min(2),     // Top margin
      Constraint::Length(22), // Logo
      Constraint::Length(3),  // Spacer
      Constraint::Length(8),  // Menu items
      Constraint::Min(2),     // Bottom margin
    ])
    .split(h_chunks[0]);

  let logo_raw = include_str!("../../assets/logo.txt");
  let logo_lines = parse_logo(logo_raw);
  let logo = Paragraph::new(logo_lines).alignment(Alignment::Center);
  f.render_widget(logo, left_chunks[1]);

  let list_h_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(30),
      Constraint::Percentage(40),
      Constraint::Percentage(30),
    ])
    .split(left_chunks[3]);

  let items = vec![
    ListItem::new("Select Map"),
    ListItem::new("How to Play"),
    ListItem::new("Quit"),
  ];

  let list = List::new(items)
    .highlight_style(
      Style::default()
        .fg(Color::Black)
        .bg(Color::Rgb(57, 211, 83))
        .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

  let mut state = ratatui::widgets::ListState::default();
  state.select(Some(app.ui.selected_index));
  f.render_stateful_widget(list, list_h_chunks[1], &mut state);

  // Right Column: Info Panel
  let right_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Min(1), // Fill
    ])
    .split(h_chunks[1]);

  let info_text = vec![
    Line::from(""),
    Line::from(Span::styled(
      "Welcome to Vim-nite!",
      Style::default()
        .fg(Color::Rgb(57, 211, 83))
        .add_modifier(Modifier::BOLD),
    )),
    Line::from(""),
    Line::from("The terminal platformer powered"),
    Line::from("by Vim motions."),
    Line::from(""),
    Line::from(""),
    Line::from(Span::styled(
      "Controls:",
      Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    )),
    Line::from("  j : Move Down"),
    Line::from("  k : Move Up"),
    Line::from("  h : Move Left / Prev Slide"),
    Line::from("  l : Move Right / Next Slide"),
    Line::from("  Enter : Select / Action"),
    Line::from("  q : Quit / Go Back"),
  ];

  let info_panel = Paragraph::new(info_text).alignment(Alignment::Left).block(
    Block::default()
      .borders(Borders::LEFT)
      .border_style(Style::default().fg(Color::Rgb(57, 211, 83)))
      .padding(Padding::new(4, 2, 4, 0)),
  );

  f.render_widget(info_panel, right_chunks[0]);
}
