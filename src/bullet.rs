use crate::{assets::BulletAsset, shooter::Shooter};
use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct BulletTimer(Timer);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, (spawn_bullets, move_bullets));
    }
}

fn spawn_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<BulletTimer>,
    query: Query<&Transform, With<Shooter>>,
    asset: Res<BulletAsset>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for transform in &query {
            commands.spawn((
                Bullet,
                Mesh2d(asset.mesh.clone()),
                MeshMaterial2d(asset.material.clone()),
                (*transform),
            ));
        }
    }
}

fn move_bullets(time: Res<Time>, q: Query<&mut Transform, With<Bullet>>) {
    let speed = 200.0;
    for mut transform in q {
        transform.translation.y += speed * time.delta_secs();
    }
}
