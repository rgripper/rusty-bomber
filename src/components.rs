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
#[derive(Debug)]
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
pub struct PlayerSensor;
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
use bevy_rapier2d::na::Vector2;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}
impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            // rand 0.8
            0 => Direction::Left,
            1 => Direction::Up,
            2 => Direction::Right,
            _ => Direction::Down,
        }
    }
}
impl Direction {
    pub fn into_dir(&self) -> Vector2<f32> {
        match self {
            Direction::Up => Vector2::new(-1.0, 0.0),
            Direction::Left => Vector2::new(0.0, 1.0),
            Direction::Down => Vector2::new(1.0, 0.0),
            Direction::Right => Vector2::new(0.0, -1.0),
        }
    }
}

pub const NEXT_PLAYER_SHEET: u32 = 14;

pub struct AnimateIndexs<T> {
    pub left: Vec<u32>,
    pub right: Vec<u32>,
    pub up: Vec<u32>,
    pub down: Vec<u32>,
    _mark: PhantomData<T>,
}
impl<T> AnimateIndexs<T> {
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
            _mark: PhantomData::default(),
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
            _mark: PhantomData::default(),
        }
    }
    pub fn player3() -> Self {
        let left = vec![38, 39, 40, 41];
        let right = vec![32, 33, 34, 35];
        let up = vec![28, 36, 37];
        let down = vec![29, 30, 31];
        Self {
            left,
            right,
            up,
            down,
            _mark: PhantomData::default(),
        }
    }
    pub fn player4() -> Self {
        let left = vec![52, 53, 54, 55];
        let right = vec![46, 47, 48, 49];
        let up = vec![42, 50, 51];
        let down = vec![43, 44, 45];
        Self {
            left,
            right,
            up,
            down,
            _mark: PhantomData::default(),
        }
    }
}
