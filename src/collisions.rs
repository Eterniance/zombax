use bevy::prelude::*;

#[derive(Component)]
pub enum HitBox {
    Circle { radius: f32 },
    Box { length: f32, width: f32 },
}

pub fn collides(position_1: Vec2, hitbox_1: &HitBox, position_2: Vec2, hitbox_2: &HitBox) -> bool {
    match (hitbox_1, hitbox_2) {
        (HitBox::Circle { radius: r1 }, HitBox::Circle { radius: r2 }) => {
            position_1.distance_squared(position_2) <= (r1 + r2).powi(2)
        }
        (HitBox::Circle { radius }, HitBox::Box { length, width }) => {
            circle_box_collision(position_1, position_2, *radius, *length, *width)
        }
        (HitBox::Box { length, width }, HitBox::Circle { radius }) => {
            circle_box_collision(position_2, position_1, *radius, *length, *width)
        }
        (
            HitBox::Box {
                length: l1,
                width: w1,
            },
            HitBox::Box {
                length: l2,
                width: w2,
            },
        ) => {
            let Vec2 { x: x1, y: y1 } = position_1;
            let Vec2 { x: x2, y: y2 } = position_2;
            (x1 - x2).abs() <= l1 / 2.0 + l2 / 2.0 && (y1 - y2).abs() <= w1 / 2.0 + w2 / 2.0
        }
    }
}

fn circle_box_collision(
    circle_center: Vec2,
    box_center: Vec2,
    radius: f32,
    length: f32,
    width: f32,
) -> bool {
    let half_width = width / 2.0;
    let half_length = length / 2.0;

    let closest_x = circle_center
        .x
        .clamp(box_center.x - half_length, box_center.x + half_length);

    let closest_y = circle_center
        .y
        .clamp(box_center.y - half_width, box_center.y + half_width);

    let closest_point = Vec2::new(closest_x, closest_y);

    circle_center.distance(closest_point) <= radius
}
