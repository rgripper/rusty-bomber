pub use bevy::prelude::*;
// Bundle
#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    direction: Direction,
    velocity: Velocity,
    destructable: Destructable,
    bomb_power: BombPower,
    bomb_number: BombNumber,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            direction: Direction::Right,
            velocity: Velocity {
                current: 0.0,
                max: 2.0,
            },
            destructable: Destructable::Player,
            bomb_power: BombPower(1),
            bomb_number: BombNumber { max: 1, current: 0 },
        }
    }
}
// Component
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
pub struct Player;
pub struct Velocity {
    max: f32,
    current: f32,
}

pub struct Creature;
pub struct Bomb {
    timer: Timer,
    player: Entity,
}

pub struct BombPower(i32);
pub struct BombNumber {
    max: i32,
    current: i32,
}
impl BombNumber {
    pub fn is_enough(&self) -> bool {
        self.current < self.max
    }
}
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
pub struct PowerBuffMaterial(Handle<ColorMaterial>);
pub struct SpeedBuffMaterial(Handle<ColorMaterial>);
pub struct BombNumberBuffMaterial(Handle<ColorMaterial>);
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
pub struct HavePlayerWayEvent(Vec3);

pub enum FixedMoveEvent {
    HaveWay(Vec3, bool),
    NoWay,
}
pub struct SmoothMoveEvent(Vec3);

pub struct ExistsEvent;
pub struct PlantBombEvent(Vec3);
pub struct RequestSpawnBombEvent(Vec3);
pub struct GameOverEvent(Entity);
pub struct RecoveryBombNumberEvent(Entity);
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
        .add_event::<HavePlayerWayEvent>()
        .add_event::<ExistsEvent>()
        .add_event::<PlantBombEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<RecoveryBombNumberEvent>()
        .add_startup_stage(GMAE_SETUP, SystemStage::parallel())
        .add_stage(MOVEMENT, SystemStage::parallel())
        .add_stage_before(MOVEMENT, BASES, SystemStage::parallel()) // <--
        .add_stage_after(BASES, BOMB, SystemStage::parallel())
        .add_stage_after(BASES, BUFF, SystemStage::parallel())
        .add_startup_system_to_stage(GMAE_SETUP, game_setup_room.system())
        .add_system_to_stage(BASES, have_player_way_position.system())
        .add_system_to_stage(MOVEMENT, change_direction.system())
        .add_system_to_stage(MOVEMENT, player_movement.system())
        .add_system_to_stage(BOMB, space_to_set_bomb.system())
        .add_system_to_stage(BOMB, bomb_trigger.system())
        .add_system_to_stage(BOMB, despawn_fire.system())
        .add_system_to_stage(BOMB, bomb_block_player.system())
        .add_system_to_stage(BOMB, bomb_destruction.system())
        .add_system_to_stage(BOMB, recovery_bomb_number.system())
        .add_system_to_stage(BUFF, buffs.system())
        .run();
}
