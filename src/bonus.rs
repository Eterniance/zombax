use bevy::prelude::*;

use crate::{
    assets::BonusAsset,
    shooter::{MainShooter, SpawnShooter},
};

#[derive(Component)]
pub struct Bonus;

#[derive(Resource)]
pub struct SpawnBonusTimer(Timer);

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnBonusTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (move_bonus, spawn_bonus, despawn_bonus, collisions));
    }
}

pub fn spawn_bonus(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnBonusTimer>,
    bonus_asset: Res<BonusAsset>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Bonus,
            Mesh2d(bonus_asset.mesh.clone()),
            MeshMaterial2d(bonus_asset.material.clone()),
            Transform::from_xyz(400.0, 400.0, 0.0),
        ));
    }
}

pub fn move_bonus(q: Query<&mut Transform, With<Bonus>>) {
    for mut transform in q {
        transform.translation.y -= 1.5;
    }
}

pub fn collisions(
    mut commands: Commands,
    bonus_query: Query<(Entity, &Transform), With<Bonus>>,
    shooter_query: Query<&Transform, With<MainShooter>>,
    mut message_writer: MessageWriter<SpawnShooter>,
) {
    for (bonus_entity, bonus_transform) in &bonus_query {
        for shooter_transform in &shooter_query {
            if detect_collision(
                &shooter_transform.translation,
                &bonus_transform.translation,
                50.0,
            ) {
                message_writer.write(SpawnShooter);
                commands.entity(bonus_entity).despawn();
                break;
            }
        }
    }
}

pub fn despawn_bonus(mut commands: Commands, pos: Query<(Entity, &Transform), With<Bonus>>) {
    for (entity, transform) in pos {
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn detect_collision(pos1: &Vec3, pos2: &Vec3, treshold: f32) -> bool {
    let Vec3 { x: x1, y: y1, z: _ } = pos1;

    let Vec3 { x: x2, y: y2, z: _ } = pos2;

    (x1 - x2).abs() < treshold && (y1 - y2).abs() < treshold
}
