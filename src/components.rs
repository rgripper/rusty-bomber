use std::marker::PhantomData;

use bevy::prelude::{Entity, Timer};

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
pub enum Destructible {
    NormalBox,
    PowerBuffBox,
    SpeedBuffBox,
    BombNumberBuffBox,
    Portal,
    Player,
    Creature,
}
pub struct MaxAndCurrent(i32, i32);
pub struct Player {
    pub is_moving: bool,
}
pub struct Portal;
pub struct Stop;
pub struct Velocity(pub f32);

pub struct Bomb {
    pub timer: Timer,
    pub player: Entity,
}

impl Default for Bomb {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, false),
            player: Entity::new(0),
        }
    }
}

pub struct InGame;

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}
pub const NEXT_PLAYER_SHEET: u32 = 14;

pub struct AnimateData<T> {
    pub left: Vec<u32>,
    pub right: Vec<u32>,
    pub up: Vec<u32>,
    pub down: Vec<u32>,
    _photo_data: PhantomData<T>,
}
impl<T> AnimateData<T> {
    pub fn player1() -> Self {
        let left = vec![10, 11, 12, 13];
        let right = vec![4, 5, 6, 7];
        let up = vec![0, 8, 9];
        let down = vec![1, 2, 3];
        Self {
            left,
            right,
            up,
            down,
            _photo_data: PhantomData::default(),
        }
    }
    pub fn player2() -> Self {
        let left = vec![24, 25, 26, 27];
        let right = vec![18, 19, 20, 21];
        let up = vec![14, 22, 23];
        let down = vec![15, 16, 17];
        Self {
            left,
            right,
            up,
            down,
            _photo_data: PhantomData::default(),
        }
    }
    pub fn player3() -> Self {
        let left = vec![24 + 14, 25 + 14, 26 + 14, 27 + 14];
        let right = vec![18 + 14, 19 + 14, 20 + 14, 21 + 14];
        let up = vec![14 + 14, 22 + 14, 23 + 14];
        let down = vec![15 + 14, 16 + 14, 17 + 14];
        Self {
            left,
            right,
            up,
            down,
            _photo_data: PhantomData::default(),
        }
    }
    pub fn player4() -> Self {
        let left = vec![24 + 14 + 14, 25 + 14 + 14, 26 + 14 + 14, 27 + 14 + 14];
        let right = vec![18 + 14 + 14, 19 + 14 + 14, 20 + 14 + 14, 21 + 14 + 14];
        let up = vec![14 + 14 + 14, 22 + 14 + 14, 23 + 14 + 14];
        let down = vec![15 + 14 + 14, 16 + 14 + 14, 17 + 14 + 14];
        Self {
            left,
            right,
            up,
            down,
            _photo_data: PhantomData::default(),
        }
    }
}
