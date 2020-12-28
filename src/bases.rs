use crate::*;

pub const BASES: &str = "bases";

pub fn have_player_way_position(
    player_position_queue: Query<&Transform, With<Player>>,
    way_position_queue: Query<&Transform, With<Way>>,
    mut event: ResMut<Events<HavePlayerWayEvent>>,
) {
    for player in player_position_queue.iter() {
        let player_position = player.translation;
        for way in way_position_queue.iter() {
            let way_position = way.translation;
            if aabb_detection(player_position.x, player_position.y, way_position) {
                event.send(HavePlayerWayEvent(way_position));
            }
        }
    }
}
