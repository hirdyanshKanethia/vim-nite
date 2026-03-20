use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  widgets::Paragraph,
};

fn format_duration(duration: std::time::Duration) -> String {
  let mins = duration.as_secs() / 60;
  let secs = duration.as_secs() % 60;
  let millis = duration.subsec_millis();

  format!("{:02}:{:02}.{:03}", mins, secs, millis)
}

pub fn render(f: &mut Frame, area: Rect, app: &crate::app::App) {
  // Create a horizontal layout for the bar
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Length(12),
      Constraint::Min(10),
      Constraint::Length(20),
    ])
    .split(area);

  // Extract data safely from app.game (Option)
  let (map_name, lives, timer_str) = if let Some(game) = &app.game {
    (
      game.map_name.as_str(),
      format!("{} ❤", game.player.lives),
      format_duration(game.timer.elapsed()),
    )
  } else {
    ("", String::new(), String::new())
  };

  // Render Mode (Left)
  let mode_style = Style::default()
    .fg(Color::Black)
    .bg(Color::Blue)
    .add_modifier(Modifier::BOLD);
  let mode_widget = Paragraph::new("-- NORMAL --").style(mode_style);
  f.render_widget(mode_widget, chunks[0]);

  // Render Map Name (Middle-ish)
  let map_widget = Paragraph::new(format!("  {}", map_name))
    .style(Style::default().bg(Color::DarkGray).fg(Color::White));
  f.render_widget(map_widget, chunks[1]);

  // Render Timer & Lives (Right)
  let right_content = format!("{} | {} ", timer_str, lives);
  let right_widget = Paragraph::new(right_content)
    .alignment(ratatui::layout::Alignment::Right)
    .style(Style::default().bg(Color::DarkGray).fg(Color::Cyan));
  f.render_widget(right_widget, chunks[2]);
}
