use crate::{
    assets::BulletAsset,
    collisions::{HitBox, collides},
    shooter::Shooter,
    zombie::Zombie,
};
use bevy::prelude::*;

pub const BULLET_RADIUS: f32 = 1.0;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct BulletTimer(Timer);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(
                Update,
                (spawn_bullets, move_bullets, detect_bullet_collision),
            );
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
                HitBox::Circle {
                    radius: BULLET_RADIUS,
                },
            ));
        }
        commands.spawn((
            AudioPlayer::new(asset.sound.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}

fn move_bullets(time: Res<Time>, q: Query<&mut Transform, With<Bullet>>) {
    let speed = 200.0;
    for mut transform in q {
        transform.translation.y += speed * time.delta_secs();
    }
}

fn detect_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &HitBox), With<Bullet>>,
    zombie_query: Query<(Entity, &Transform, &HitBox), With<Zombie>>,
) {
    for (bullet, b_transform, b_hitbox) in &bullet_query {
        for (zombie, z_transform, z_hitbox) in &zombie_query {
            if collides(
                b_transform.translation.truncate(),
                b_hitbox,
                z_transform.translation.truncate(),
                z_hitbox,
            ) {
                commands.entity(bullet).despawn();
                commands.entity(zombie).despawn();
                break;
            }
        }
    }
}
