use bevy::prelude::*;

use crate::{
    assets::ZombieAsset,
    types::{SpawnTimer, Zombie},
};

pub struct ZombiePlugin;

impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Update, (eternal_walk, endless_spawn));
    }
}

fn eternal_walk(q: Query<&mut Transform, With<Zombie>>) {
    for mut transform in q {
        transform.translation.y -= 1.0;
    }
}

fn endless_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    zombie_asset: Res<ZombieAsset>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for x in [-200.0, -100.0, 100.0, 200.0] {
            commands.spawn((
                Zombie,
                Mesh2d(zombie_asset.mesh.clone()),
                MeshMaterial2d(zombie_asset.material.clone()),
                Transform::from_xyz(x, 200.0, 0.0),
            ));
        }
    }
}
