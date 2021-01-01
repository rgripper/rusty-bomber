use bevy::prelude::*;

use crate::components::{Animation, Direction, Player, PlayerAnimation, Velocity};

pub trait AnimateSystems {
    fn animate_systems(&mut self) -> &mut Self;
}
impl AnimateSystems for SystemStage {
    fn animate_systems(&mut self) -> &mut Self {
        self.add_system(animate_player.system())
            .add_system(velocity_to_animation.system())
    }
}
fn animate_player(
    time: Res<Time>,
    // texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Animation,
        &mut TextureAtlasSprite,
        &Velocity,
        &Direction,
    )>,
) {
    for (mut animation, mut sprite, _, direction) in query
        .iter_mut()
        .filter(|(_, _, velocity, _)| velocity.current != 0.0)
    {
        let indexs = PlayerAnimation::from(*direction).indexs;
        let mut should_turn = true;
        'contatine: for &idx in indexs.iter() {
            if sprite.index == idx {
                should_turn = false;
                break 'contatine;
            }
        }
        if should_turn {
            sprite.index = indexs[0];
        }
        animation.0.tick(time.delta_seconds());
        if animation.0.just_finished() {
            let indexs = PlayerAnimation::from(*direction).indexs;
            if sprite.index == indexs[0] {
                sprite.index = indexs[1];
            } else if sprite.index == indexs[1] {
                sprite.index = indexs[2];
            } else {
                sprite.index = indexs[0];
            }
            //info!("index:{}", sprite.index);
        }
    }
}
fn velocity_to_animation(
    mut query: Query<(&Velocity, &mut Animation), (With<Player>, Changed<Velocity>)>,
) {
    for (velocity, mut animation) in query.iter_mut() {
        animation.0.set_duration(1.0 / velocity.max * 0.5);
    }
}
