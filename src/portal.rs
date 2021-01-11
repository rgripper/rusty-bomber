use crate::{
    components::{Player, PlayerPosition, Portal, Stop},
    events::*,
    utils::vecs_xy_intersect,
};
use bevy::ecs::{Query, ResMut, SystemStage, With};
use bevy::prelude::*;

pub trait PortalSystems {
    fn portal_systems(&mut self) -> &mut Self;
}
impl PortalSystems for SystemStage {
    fn portal_systems(&mut self) -> &mut Self {
        self.add_system(portal_player_collision.system())
    }
}

fn portal_player_collision(
    commands: &mut Commands,
    mut player_query: Query<(Entity, &mut PlayerPosition), With<Player>>,
    mut portal_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Portal>>,
    mut game_over_events: ResMut<Events<GameOverEvent>>,
) {
    for (entity, player) in player_query.iter_mut() {
        let player_pos = &player.truncate();
        for (portal_transform, mut sprite_index) in portal_query.iter_mut() {
            if vecs_xy_intersect(&portal_transform.translation.truncate(), player_pos) {
                sprite_index.index = 1;
                commands.insert_one(entity, Stop);
                game_over_events.send(GameOverEvent(GameOverType::Victory));
                // TODO: stop the game (stop movement system?)
            }
        }
    }
}
