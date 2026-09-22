use bevy::{
    // diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};
use zombax::{
    assets::{AssetInitSet, AssetsPlugin},
    bonus::BonusPlugin,
    bullet::BulletPlugin,
    shooter::ShooterPlugin,
    zombie::ZombiePlugin,
};
pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // FrameTimeDiagnosticsPlugin::default(),
            // LogDiagnosticsPlugin::default(),
            // EntityCountDiagnosticsPlugin::default(),
        ))
        .add_plugins(AssetsPlugin)
        .add_plugins((ZombiePlugin, BonusPlugin, ShooterPlugin, BulletPlugin))
        .add_systems(Startup, setup.after(AssetInitSet))
        .add_systems(Update, debug_position)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn debug_position(
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
