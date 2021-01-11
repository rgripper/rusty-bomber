use std::ops::Deref;

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
    NormalBox,
    PowerBuffBox,
    SpeedBuffBox,
    BombNumberBuffBox,
    Portal,
}
pub struct MaxAndCurrent(i32, i32);
pub struct Player;
pub struct Portal;
pub struct Stop;
pub struct Velocity {
    pub max: f32,
    pub current: f32,
}

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
pub const FIRE_LIFETIME: f32 = 0.5;
pub const FIRE_ANIMATE_TIME: f32 = 0.05;
pub const EMBER_LIFETIME: f32 = FIRE_LIFETIME - FIRE_ANIMATE_TIME * 3.0;
pub const EMBER_START_TIME: f32 = FIRE_LIFETIME - EMBER_LIFETIME;
pub struct Fire(pub Timer);
impl Default for Fire {
    fn default() -> Self {
        Fire(Timer::from_seconds(FIRE_LIFETIME, false))
    }
}
impl Fire {
    pub fn ember() -> Self {
        Fire(Timer::from_seconds(EMBER_LIFETIME, false))
    }
}
pub struct Ember(pub Timer, pub i32);
impl Ember {
    pub fn new(power: i32) -> Self {
        Ember(Timer::from_seconds(EMBER_START_TIME, false), power)
    }
}
pub struct Dizziness(Timer, f32);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}
pub const NEXT_PLAYER_SHEET: u32 = 14;
pub struct PlayerAnimation {
    pub indexs: Vec<u32>,
}
impl PlayerAnimation {
    pub fn new(indexs: Vec<u32>) -> Self {
        Self { indexs }
    }
    pub fn next(self) -> Self {
        Self::new(
            self.indexs
                .into_iter()
                .map(|num| num + NEXT_PLAYER_SHEET)
                .collect(),
        )
    }
}
impl From<Direction> for PlayerAnimation {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Left => PlayerAnimation::new(vec![10, 11, 12, 13]),
            Direction::Up => PlayerAnimation::new(vec![0, 8, 9]),
            Direction::Right => PlayerAnimation::new(vec![4, 5, 6, 7]),
            Direction::Down => PlayerAnimation::new(vec![1, 2, 3]),
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
impl Deref for PlayerPosition {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
