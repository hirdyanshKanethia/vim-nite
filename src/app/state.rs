#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
  MainMenu,
  MapSelect,
  Playing,
  Paused,
  Message(GameEvent),
  Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
  Checkpoint,
  Death,
  Lost,
  Won,
}
