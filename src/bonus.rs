use bevy::prelude::*;

use crate::assets::BonusAsset;

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
        .add_systems(Update, (move_bonus, spawn_bonus, despawn_bonus));
    }
}

fn spawn_bonus(
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

fn move_bonus(q: Query<&mut Transform, With<Bonus>>) {
    for mut transform in q {
        transform.translation.y -= 1.5;
    }
}

fn despawn_bonus(mut commands: Commands, pos: Query<(Entity, &Transform), With<Bonus>>) {
    for (entity, transform) in pos {
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn();
        }
    }
}
