use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum BPGameState {
    #[default]
    InGame,
    GameOver,
    Victory,
}