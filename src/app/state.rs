#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
  MainMenu,
  MapSelect,
  Playing,
  Paused,
  Message(Event),
  EnteringCommand,
  Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
  GameCheckpointReached,
  PlayerDeath,
  GameLost,
  GameWon,
  InvalidMap,
}
