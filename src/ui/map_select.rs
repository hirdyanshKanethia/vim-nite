use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout, Alignment},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Padding},
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
  let size = f.area();

  // Global Border Block
  let global_block = Block::default()
      .title(" [ MAP SELECT ] ")
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .border_style(Style::default().fg(Color::Rgb(57, 211, 83)));
  
  let inner_area = global_block.inner(size);
  f.render_widget(global_block, size);

  let h_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(50),
      Constraint::Percentage(50),
    ])
    .split(inner_area);

  // Left side: The List
  let list_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Min(1), // fill
    ])
    .split(h_chunks[0]);

  let items: Vec<ListItem> = app
    .available_maps
    .iter()
    .map(|info| {
      let name_color = if info.is_valid { Color::White } else { Color::DarkGray };
      
      let line = Line::from(vec![
        Span::styled(
          info.name.clone(),
          Style::default().fg(name_color).add_modifier(Modifier::BOLD),
        ),
      ]);
      ListItem::new(line)
    })
    .collect();

  let list = List::new(items)
    .block(
      Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(Color::Rgb(57, 211, 83)))
        .padding(Padding::new(2, 2, 2, 2)),
    )
    .highlight_style(
      Style::default()
        .fg(Color::Black)
        .bg(Color::Rgb(57, 211, 83))
        .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol("❯ ");

  let mut state = ratatui::widgets::ListState::default();
  state.select(Some(app.ui.selected_index));

  f.render_stateful_widget(list, list_chunks[0], &mut state);

  // Right side: Details panel
  let mut details_lines = Vec::new();
  
  if let Some(info) = app.available_maps.get(app.ui.selected_index) {
      details_lines.push(Line::from(""));
      details_lines.push(Line::from(vec![
          Span::styled("Map: ", Style::default().fg(Color::DarkGray)),
          Span::styled(info.name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
      ]));
      details_lines.push(Line::from(""));
      
      if info.is_valid {
          details_lines.push(Line::from(vec![
              Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
              Span::styled("Valid Playable Map", Style::default().fg(Color::Green)),
          ]));
          details_lines.push(Line::from(""));
          
          if info.completions > 0 {
              details_lines.push(Line::from(vec![
                  Span::styled("Best Time: ", Style::default().fg(Color::DarkGray)),
                  Span::styled(format!("{:.3}s", info.best_time_ms.unwrap_or(0) as f64 / 1000.0), Style::default().fg(Color::Yellow)),
              ]));
              details_lines.push(Line::from(vec![
                  Span::styled("Completions: ", Style::default().fg(Color::DarkGray)),
                  Span::styled(format!("{}", info.completions), Style::default().fg(Color::Cyan)),
              ]));
          } else {
              details_lines.push(Line::from(vec![
                  Span::styled("Best Time: ", Style::default().fg(Color::DarkGray)),
                  Span::styled("--:--.---", Style::default().fg(Color::DarkGray)),
              ]));
              details_lines.push(Line::from(vec![
                  Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
                  Span::styled("Unbeaten", Style::default().fg(Color::Yellow)),
              ]));
          }
      } else {
          details_lines.push(Line::from(vec![
              Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
              Span::styled("Invalid Map Data", Style::default().fg(Color::Red)),
          ]));
      }
  }

  let details = Paragraph::new(details_lines)
      .alignment(Alignment::Left)
      .block(Block::default().padding(Padding::new(4, 2, 2, 2)));

  f.render_widget(details, h_chunks[1]);
}
