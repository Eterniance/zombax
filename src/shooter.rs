use bevy::prelude::*;

use crate::assets::{AssetInitSet, ShooterAsset};

#[derive(Component)]
pub struct Shooter;

pub struct ShooterPlugin;

impl Plugin for ShooterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shooter.after(AssetInitSet));
    }
}

fn spawn_shooter(mut commands: Commands, shooter_asset: Res<ShooterAsset>) {
    commands.spawn((
        Shooter,
        Mesh2d(shooter_asset.mesh.clone()),
        MeshMaterial2d(shooter_asset.material.clone()),
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));
}
