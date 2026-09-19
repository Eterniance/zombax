use bevy::prelude::*;
use zombax::{
    assets::{AssetInitSet, AssetsPlugin, ShooterAsset},
    game::ZombiePlugin,
    types::Shooter,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetsPlugin)
        .add_plugins(ZombiePlugin)
        .add_systems(Startup, setup.after(AssetInitSet))
        .run();
}

fn setup(mut commands: Commands, shooter_asset: Res<ShooterAsset>) {
    commands.spawn(Camera2d);

    // Player
    commands.spawn((
        Shooter,
        Mesh2d(shooter_asset.mesh.clone()),
        MeshMaterial2d(shooter_asset.material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
