use crate::{
    components::{Player, PlayerPosition},
    events::*,
    utils::vecs_xy_intersect,
};
use bevy::prelude::*;
use bevy::{
    ecs::{Query, ResMut, SystemStage, With},
    sprite::ColorMaterial,
};

pub struct PortalMaterial(pub Handle<ColorMaterial>);

pub struct Portal;

pub trait PortalSystems {
    fn portal_systems(&mut self) -> &mut Self;
}
impl PortalSystems for SystemStage {
    fn portal_systems(&mut self) -> &mut Self {
        self.add_system(portal_player_collision.system())
    }
}

fn portal_player_collision(
    mut player_query: Query<&mut PlayerPosition, With<Player>>,
    mut portal_query: Query<&mut Transform, With<Portal>>,
    mut game_over_events: ResMut<Events<GameOverEvent>>,
) {
    for player in player_query.iter_mut() {
        let player_pos = &player.truncate();
        for portal_transform in portal_query.iter_mut() {
            if vecs_xy_intersect(&portal_transform.translation.truncate(), player_pos) {
                game_over_events.send(GameOverEvent(GameOverType::Victory));
                // TODO: stop the game (stop movement system?)
            }
        }
    }
}
