use bevy::prelude::*;

use crate::assets::{AssetInitSet, ShooterAsset};

#[derive(Component)]
pub struct Shooter;

pub struct ShooterPlugin;

impl Plugin for ShooterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shooter.after(AssetInitSet))
            .add_systems(Update, move_shooter);
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

fn move_shooter(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    q: Query<&mut Transform, With<Shooter>>,
) {
    let speed = 200.0;

    let mut direction = 0.0;

    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    for mut transform in q {
        transform.translation.x += direction * speed * time.delta_secs();
    }
}
