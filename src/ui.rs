pub mod main_menu;
// pub mod map_select;
// pub mod pause_menu;

use crate::app::App;
use ratatui::Frame;

pub fn render_main_menu(f: &mut Frame, app: &App) {
  main_menu::render(f, app);
}

// pub fn render_map_select<B: Backend>(f: &mut Frame<B>, app: &App) {
//   map_select::render(f, app);
// }
//
// pub fn render_pause_menu<B: Backend>(f: &mut Frame<B>, app: &App) {
//   pause_menu::render(f, app);
// }
