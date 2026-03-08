#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
  MainMenu,
  MapSelect,
  Playing,
  Paused,
  Message(MessageType),
  Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
  Checkpoint,
  Death,
  Lost,
}
