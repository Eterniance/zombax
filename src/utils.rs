use bevy::math::Vec3;

pub fn detect_collision(pos1: &Vec3, pos2: &Vec3, treshold: f32) -> bool {
    let Vec3 { x: x1, y: y1, z: _ } = pos1;

    let Vec3 { x: x2, y: y2, z: _ } = pos2;

    (x1 - x2).abs() < treshold && (y1 - y2).abs() < treshold
}