use bevy::prelude::{ColorMaterial, Handle};

pub struct LifeMaterial(pub Handle<ColorMaterial>);
pub struct PermaWallMaterial(pub Handle<ColorMaterial>);
pub struct DestructableWallMaterial(pub Handle<ColorMaterial>);

pub struct FloorMaterial(pub Handle<ColorMaterial>);

pub struct PlayerMaterial(pub Handle<ColorMaterial>);

pub struct BombMaterial(pub Handle<ColorMaterial>);

pub struct CreatureMaterial(pub Handle<ColorMaterial>);

pub struct FireMaterial(pub Handle<ColorMaterial>);

pub struct PowerBuffMaterial(pub Handle<ColorMaterial>);

pub struct SpeedBuffMaterial(pub Handle<ColorMaterial>);

pub struct BombNumberBuffMaterial(pub Handle<ColorMaterial>);
