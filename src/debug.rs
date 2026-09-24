use bevy::{prelude::*, window::PrimaryWindow};

use crate::collisions::HitBox;

#[derive(Resource)]
struct DebugHitboxes(bool);

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugHitboxes(false)).add_systems(
            Update,
            (
                debug_position,
                debug_hitboxes.run_if(debug_hitboxes_enabled),
                toggle_debug_hitbox,
            ),
        );
    }
}

fn debug_hitboxes_enabled(debug_hitboxes: Res<DebugHitboxes>) -> bool {
    debug_hitboxes.0
}

fn toggle_debug_hitbox(mut toggle: ResMut<DebugHitboxes>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::F2) {
        toggle.0 = !toggle.0;
    }
}

fn debug_position(
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    if keyboard.just_pressed(KeyCode::F6)
        && let Some(cursor_position) = window.cursor_position()
        && let Ok(world_position) = camera.0.viewport_to_world_2d(camera.1, cursor_position)
    {
        println!("vec2({},{}),", world_position.x, world_position.y);
    }
}

fn debug_hitboxes(mut gizmos: Gizmos, query: Query<(&Transform, &HitBox)>) {
    for (transform, hitbox) in query {
        let position = transform.translation.truncate();

        match hitbox {
            HitBox::Box { length, width } => {
                gizmos.rect_2d(
                    position,
                    Vec2 {
                        x: *length,
                        y: *width,
                    },
                    Color::srgb(1.0, 0.0, 0.0),
                );
            }
            HitBox::Circle { radius } => {
                gizmos.circle_2d(position, *radius, Color::srgb(1.0, 0.0, 0.0));
            }
        }
    }
}
