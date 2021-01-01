use bevy::{
    math::{Vec2, Vec3},
    prelude::{Entity, Timer},
};

use crate::constants::PLAYER_LAYER;

pub struct Wall;
pub struct Way;
pub struct SpeedWay;
pub struct Animation(pub Timer);

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
pub struct Player;
pub struct Velocity {
    pub max: f32,
    pub current: f32,
}

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
pub struct PlayerAnimation {
    pub indexs: [u32; 3],
}
impl PlayerAnimation {
    pub fn new(indexs: [u32; 3]) -> Self {
        Self { indexs }
    }
}
impl From<Direction> for PlayerAnimation {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Left => PlayerAnimation::new([0, 4, 8]),
            Direction::Up => PlayerAnimation::new([2, 6, 10]),
            Direction::Right => PlayerAnimation::new([3, 7, 11]),
            Direction::Down => PlayerAnimation::new([1, 5, 9]),
        }
    }
}
pub struct PlayerPosition(pub Vec3);

impl From<Vec3> for PlayerPosition {
    fn from(v: Vec3) -> Self {
        PlayerPosition(v)
    }
}
impl From<Vec2> for PlayerPosition {
    fn from(v: Vec2) -> Self {
        PlayerPosition(v.extend(PLAYER_LAYER))
    }
}
