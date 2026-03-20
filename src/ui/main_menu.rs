use crate::app::App;
use ratatui::{
  Frame,
  layout::{Alignment, Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
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
          // style changed, flush current span
          let style = if current_is_block { im_style } else { v_style };
          spans.push(Span::styled(current.clone(), style));
          current.clear();
          current_is_block = char_is_block;
          current.push(c);
        }
      }

      // flush remaining
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

  let logo_raw = include_str!("../../assets/logo.txt");
  let logo_lines = parse_logo(logo_raw);

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Length(5),
      Constraint::Length(22),
      Constraint::Length(5),
      Constraint::Length(6),
      Constraint::Min(0),
    ])
    .split(size);

  let logo = Paragraph::new(logo_lines).alignment(Alignment::Center);
  f.render_widget(logo, chunks[1]);

  let items = vec![ListItem::new("Select Map"), ListItem::new("Quit")];

  let list = List::new(items)
    .block(
      Block::default()
        .title(" Main Menu ")
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Rgb(77, 124, 15)))
        .title_style(
          Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
        ),
    )
    .highlight_style(
      Style::default()
        .fg(Color::Black)
        .bg(Color::Rgb(57, 211, 83))
        .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

  let mut state = ratatui::widgets::ListState::default();
  state.select(Some(app.ui.selected_index));
  f.render_stateful_widget(list, chunks[3], &mut state);
}
