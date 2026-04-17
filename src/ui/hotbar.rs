use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  widgets::Paragraph,
};

const EMPTY_COMMAND_MESSAGES: &[&str] = &[
  "Go on, atleast try with something",
  "If you didn't know, the command you entered was empty",
  "Empty commands won't do anything, might as well write something",
];

const UNKNOWN_COMMAND_MESSAGES: &[&str] = &[
  "Find the exit broski, this isn't your ordinary vim",
  "'X' is the exit brody, this won't work. Never did",
  "nuh uh broski, getting deja vu already?",
  "nostalgic no?",
];

fn format_duration(duration: std::time::Duration) -> String {
  let mins = duration.as_secs() / 60;
  let secs = duration.as_secs() % 60;
  let millis = duration.subsec_millis();

  format!("{:02}:{:02}.{:03}", mins, secs, millis)
}

pub fn render(f: &mut Frame, status_bar_area: Rect, command_area: Rect, app: &crate::app::App) {
  let status_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Length(12),
      Constraint::Min(10),
      Constraint::Length(25),
    ])
    .split(status_bar_area);

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
  f.render_widget(
    Paragraph::new("-- NORMAL --").style(mode_style),
    status_chunks[0],
  );

  // Render Map Name (Middle)
  let map_widget = Paragraph::new(format!("  {}", map_name))
    .style(Style::default().bg(Color::DarkGray).fg(Color::White));
  f.render_widget(map_widget, status_chunks[1]);

  // Render Timer & Lives (Right)
  let right_content = format!("{} | {} ", timer_str, lives);
  let right_widget = Paragraph::new(right_content)
    .alignment(ratatui::layout::Alignment::Right)
    .style(Style::default().bg(Color::DarkGray).fg(Color::Cyan));
  f.render_widget(right_widget, status_chunks[2]);

  // Command bar at the bottom
  let command_line = Paragraph::new(app.ui.command_buffer.clone())
    .style(Style::default().bg(Color::Reset).fg(Color::White));

  f.render_widget(command_line, command_area);
}

pub fn process_command(command_buffer: &str) -> String {
  let trimmed = command_buffer.trim();

  if trimmed.is_empty() || trimmed == ":" {
    return EMPTY_COMMAND_MESSAGES[fastrand::usize(..EMPTY_COMMAND_MESSAGES.len())].to_string();
  }

  match trimmed {
    ":q" => "Use 'q' to pause, not this.".to_string(),
    _ => UNKNOWN_COMMAND_MESSAGES[fastrand::usize(..UNKNOWN_COMMAND_MESSAGES.len())].to_string(),
  }
}
