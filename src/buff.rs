use bevy::prelude::*;

use crate::{
    components::{BombNumber, BombPower, Buff, Player, Velocity},
    utils::aabb_detection,
};

pub const BUFF: &str = "BUFF";

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
            if aabb_detection(transform.translation.x, transform.translation.y, position) {
                commands.despawn(entity);
                match buff {
                    Buff::PowerBuff => {
                        power.0 += 1;
                    }
                    Buff::SpeedBuff => {
                        // TODO:
                        velocity.0 *= 1.2;

                    }
                    Buff::BombNumberBuff => {
                        number.max += 1;
                    }
                }
            }
        }
    }
}
