use ratatui::{
  Frame,
  layout::Rect,
  style::{Color, Style},
  text::{Line, Span},
  widgets::Paragraph,
};

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
  let mode = "-- NORMAL --";

  let map_name = if let Some(game) = &app.game {
    &game.map_name
  } else {
    ""
  };

  let lives = if let Some(game) = &app.game {
    format!("{} ❤", game.player.lives)
  } else {
    "".into()
  };

  let line = Line::from(vec![
    Span::styled(mode, Style::default().fg(Color::Black).bg(Color::White)),
    Span::raw("   "),
    Span::raw(map_name),
    Span::raw("   "),
    Span::raw(lives),
  ]);

  let paragraph = Paragraph::new(line).style(Style::default().bg(Color::DarkGray));

  f.render_widget(paragraph, area);
}
