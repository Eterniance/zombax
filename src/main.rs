use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use zombax::{
    assets::{AssetInitSet, AssetsPlugin},
    bonus::BonusPlugin,
    shooter::ShooterPlugin,
    zombie::ZombiePlugin,
};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
        ))
        .add_plugins(AssetsPlugin)
        .add_plugins((ZombiePlugin, BonusPlugin, ShooterPlugin))
        .add_systems(Startup, setup.after(AssetInitSet))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
