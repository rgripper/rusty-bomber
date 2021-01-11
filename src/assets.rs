use bevy::{
    prelude::{ColorMaterial, Handle},
    sprite::TextureAtlas,
};

pub struct LifeMaterial(pub Handle<ColorMaterial>);

pub struct PlayerTextureAtlas(pub Handle<TextureAtlas>);

pub struct BombTextureAtlas(pub Handle<TextureAtlas>);
pub struct FireTextureAtlas(pub Handle<TextureAtlas>);
pub struct FloorOrWallTextureAtlas(pub Handle<TextureAtlas>);
pub struct CreatureTextureAtlas(pub Handle<TextureAtlas>);
pub struct PortalTextureAtlas(pub Handle<TextureAtlas>);
pub struct PowerBuffMaterial(pub Handle<ColorMaterial>);

pub struct SpeedBuffMaterial(pub Handle<ColorMaterial>);

pub struct BombNumberBuffMaterial(pub Handle<ColorMaterial>);
