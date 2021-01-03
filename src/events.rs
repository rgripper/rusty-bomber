use bevy::prelude::Entity;

pub enum GameOverType {
    Victory,
    Defeat,
}
pub struct GameOverEvent(pub GameOverType);
pub struct RecoveryBombNumberEvent(pub Entity);
