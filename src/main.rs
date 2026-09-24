use bevy::{
    // diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use zombax::{
    assets::{AssetInitSet, AssetsPlugin},
    bonus::BonusPlugin,
    bullet::BulletPlugin,
    debug::DebugPlugin,
    shooter::ShooterPlugin,
    zombie::ZombiePlugin,
};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // FrameTimeDiagnosticsPlugin::default(),
            // LogDiagnosticsPlugin::default(),
            // EntityCountDiagnosticsPlugin::default(),
        ))
        .add_plugins(AssetsPlugin)
        .add_plugins((ZombiePlugin, BonusPlugin, ShooterPlugin, BulletPlugin))
        .add_plugins(DebugPlugin)
        .add_systems(Startup, setup.after(AssetInitSet))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
