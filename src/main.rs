use bevy::prelude::*;
use zombax::{
    assets::{AssetInitSet, AssetsPlugin, ShooterAsset, ZombieAsset}, types::{Shooter, Zombie}, game::ZombiePlugin,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetsPlugin)
        .add_plugins(ZombiePlugin)
        .add_systems(Startup, setup.after(AssetInitSet))
        .run();
}

fn setup(
    mut commands: Commands,
    zombie_asset: Res<ZombieAsset>,
    shooter_asset: Res<ShooterAsset>
) {
    commands.spawn(Camera2d);

    // Player
    commands.spawn((
        Shooter,
        Mesh2d(shooter_asset.mesh.clone()),
        MeshMaterial2d(shooter_asset.material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Zombies
    for x in [-200.0, -100.0, 100.0, 200.0] {
        commands.spawn((
            Zombie,
            Mesh2d(zombie_asset.mesh.clone()),
            MeshMaterial2d(zombie_asset.material.clone()),
            Transform::from_xyz(x, 200.0, 0.0),
        ));
    }
}
