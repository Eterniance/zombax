use crate::{assets::ZombieAsset, collisions::HitBox, types::SpawnTimer};
use bevy::prelude::*;

#[derive(Component)]
pub struct Zombie;

pub struct ZombiePlugin;

impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Update, (move_zombies, endless_spawn, despawn_zombies));
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
                Sprite {
                    image: zombie_asset.texture.clone(),
                    ..default()
                },
                Transform {
                    translation: vec3(x, 400.0, 0.0),
                    scale: Vec3::splat(50.0 / 32.0),
                    ..default()
                },
                HitBox::Box {
                    length: 50.0,
                    width: 50.0,
                },
            ));
        }
    }
}

fn move_zombies(q: Query<&mut Transform, With<Zombie>>) {
    for mut transform in q {
        transform.translation.y -= 1.0;
    }
}

fn despawn_zombies(mut commands: Commands, pos: Query<(Entity, &Transform), With<Zombie>>) {
    for (entity, transform) in pos {
        if transform.translation.y < -200.0 {
            commands.entity(entity).despawn();
        }
    }
}
