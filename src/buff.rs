use bevy::prelude::*;

use crate::{
    components::{BombNumber, BombPower, Buff, Player, Velocity},
    utils::vecs_xy_intersect,
};

pub trait BuffSystems {
    fn buff_systems(&mut self) -> &mut Self;
}
impl BuffSystems for SystemStage {
    fn buff_systems(&mut self) -> &mut Self {
        self.add_system(buffs.system())
    }
}

fn buffs(
    commands: &mut Commands,
    buff_query: Query<(Entity, &Transform, &Buff), With<Buff>>,
    mut player: Query<(&Transform, &mut BombPower, &mut BombNumber, &mut Velocity), With<Player>>,
) {
    for (player, mut power, mut number, mut velocity) in player.iter_mut() {
        let position = player.translation;
        for (entity, transform, buff) in buff_query.iter() {
            if vecs_xy_intersect(&transform.translation.truncate(), &position.truncate()) {
                commands.despawn(entity);
                match buff {
                    Buff::PowerBuff => {
                        power.0 += 1;
                    }
                    Buff::SpeedBuff => {
                        // TODO:
                        velocity.0 = (velocity.0 * 1.2).min(400.0);
                    }
                    Buff::BombNumberBuff => {
                        number.max += 1;
                    }
                }
            }
        }
    }
}
