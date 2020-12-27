pub use bevy::prelude::*;

// Component
pub struct Wall;
pub struct Way;
pub struct Destructable;
pub struct MaxAndCurrent(i32, i32);
pub struct Player {
    is_moving: bool,
}
pub struct Life {
    before: i32,
    now: i32,
}

impl Life {
    pub fn update(&mut self, value: i32) {
        self.before = self.now;
        self.now += value;
    }
    pub fn state(&self) -> PlayerState {
        match self.now.cmp(&self.before) {
            std::cmp::Ordering::Less => PlayerState::DeBuff,
            std::cmp::Ordering::Equal => PlayerState::Normal,
            std::cmp::Ordering::Greater => PlayerState::Buff,
        }
    }
}
#[derive(PartialEq, Copy, Clone)]
pub enum PlayerState {
    Normal,
    Buff,
    DeBuff,
}
pub struct Velocity(f32);
pub struct Creature;
pub struct Bomb(Timer);
pub struct BombPower(i32);
pub struct Fire(Timer);
pub struct Dizziness(Timer, f32);
#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

// Assets
pub struct PermaWallMaterial(Handle<ColorMaterial>);

pub struct DestructableWallMaterial(Handle<ColorMaterial>);

pub struct FloorMaterial(Handle<ColorMaterial>);

pub struct PlayerMaterial(Handle<ColorMaterial>);
pub struct BombMaterial(Handle<ColorMaterial>);
pub struct CreatureMaterial(Handle<ColorMaterial>);

pub struct FireMaterial(Handle<ColorMaterial>);

// Resource
pub struct Map {
    value: Vec<Vec<i32>>,
}
impl Map {
    pub fn first() -> Self {
        let room_map = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 2, 2, 0, 0, 0, 0, 2, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        Self { value: room_map }
    }

    pub fn map_value(&self) -> &Vec<Vec<i32>> {
        &self.value
    }
}

pub struct Threshold(f32);

// Events
pub struct HavePlayerWayEvent(Vec3, bool);

pub struct RequestRepairEvent(
    Vec3,      // position
    Direction, // direction
    Vec3,      // wall_position
);

pub enum FixedMoveEvent {
    HaveWay(Vec3, bool),
    NoWay,
}
pub struct SmoothMoveEvent(Vec3);

pub struct ExistsEvent;
pub struct PlantBombEvent(Vec3);
pub struct RequestSpawnBoomEvent(Vec3);

// Constant

pub const PLAYER_LAYER: f32 = 2.0;
pub const OBJECT_LAYER: f32 = 5.0;
pub const FLOOR_LAYER: f32 = 0.0;

pub mod bases;
pub mod bomb;
pub mod buff;
pub mod movement;
pub mod setup;
pub mod utils;

use bases::*;
use bomb::*;
use buff::*;
use movement::*;
use setup::*;
use utils::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_resource(Map::first())
        .add_resource(Threshold(12.0))
        .add_event::<FixedMoveEvent>()
        .add_event::<RequestRepairEvent>()
        .add_event::<HavePlayerWayEvent>()
        .add_event::<ExistsEvent>()
        .add_event::<PlantBombEvent>()
        .add_event::<RequestSpawnBoomEvent>()
        .add_startup_stage(GMAE_SETUP, SystemStage::parallel())
        .add_stage(MOVEMENT, SystemStage::parallel())
        .add_stage_before(MOVEMENT, BASES, SystemStage::parallel()) // <--
        .add_stage_after(BASES, BOMB, SystemStage::parallel())
        .add_stage_after(BASES, BUFF, SystemStage::parallel())
        .add_startup_system_to_stage(GMAE_SETUP, game_setup_room.system())
        .add_system_to_stage(BASES, have_player_way_position.system())
        .add_system_to_stage(MOVEMENT, change_direction.system())
        .add_system_to_stage(MOVEMENT, player_movement.system())
        .add_system_to_stage(MOVEMENT, road_detection.system())
        .add_system_to_stage(BOMB, space_to_set_bomb.system())
        .add_system_to_stage(BOMB, bomb_trigger.system())
        .add_system_to_stage(BOMB, despawn_fire.system())
        .add_system_to_stage(BOMB, request_spawn_bomb.system())
        .add_system_to_stage(BOMB, bomb_block_player.system())
        .add_system_to_stage(BOMB, bomb_destruction.system())
        .add_system_to_stage(BOMB, bomb_life_things.system())
        .add_system_to_stage(BUFF, add_dizziness_buff.system())
        .add_system_to_stage(BUFF, trigger_dizziness_buff.system())
        .run();
}
