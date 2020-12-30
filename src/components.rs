use bevy::prelude::{Entity, Timer};

pub struct Wall;
pub struct Way;
pub struct SpeedWay;

pub enum Buff {
    PowerBuff,
    SpeedBuff,
    BombNumberBuff,
}
pub enum GameMode {
    SinglePlayer,
    MultiPlayer,
}
pub enum Destructable {
    Player,
    NormalBox,
    PowerBuffBox,
    SpeedBuffBox,
    BombNumberBuffBox,
}
pub struct MaxAndCurrent(i32, i32);
pub struct Player {
    pub is_moving: bool,
}
pub struct Velocity(pub f32);

pub struct Creature;

pub struct Bomb {
    pub timer: Timer,
    pub player: Entity,
}

pub struct InGame;

pub struct BombPower(pub i32);
pub struct BombNumber {
    pub max: i32,
    pub current: i32,
}
impl BombNumber {
    pub fn is_enough(&self) -> bool {
        self.current < self.max
    }
}
pub struct Fire(pub Timer);
pub struct Dizziness(Timer, f32);
#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}
