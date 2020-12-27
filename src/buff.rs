use crate::*;

pub const BUFF: &str = "BUFF";

pub fn add_dizziness_buff(
    commands: &mut Commands,
    mut query: Query<(Entity, &Life, &mut Velocity)>,
) {
    for (entity, life, mut velocity) in query.iter_mut() {
        if life.now <= 0 {
            commands.despawn(entity);
        } else if life.state() == PlayerState::DeBuff {
            commands.insert_one(
                entity,
                Dizziness(Timer::from_seconds(0.3, false), velocity.0),
            );
            velocity.0 = 0.0;
        }
    }
}
pub fn trigger_dizziness_buff(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Dizziness, &mut Velocity)>,
) {
    for (entity, mut dizziness, mut velocity) in query.iter_mut() {
        if dizziness.0.tick(time.delta_seconds()).finished() {
            velocity.0 = dizziness.1 + velocity.0;
            commands.remove_one::<Dizziness>(entity);
        }
    }
}
