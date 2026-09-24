use crate::{assets::ZombieAsset, collisions::HitBox};
use bevy::prelude::*;

#[derive(Resource)]
pub struct SpawnZombieTimer(pub Timer);

#[derive(Component)]
pub struct Zombie;

pub struct ZombiePlugin;

impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnZombieTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, (move_zombies, endless_spawn, despawn_zombies));
    }
}

fn endless_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnZombieTimer>,
    zombie_asset: Res<ZombieAsset>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for x in linspace(-200.0, 200.0, 20) {
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
                HitBox::Circle { radius: 16.0 },
            ));
        }
    }
}

fn move_zombies(time: Res<Time>, q: Query<&mut Transform, With<Zombie>>) {
    let speed = 50.0;
    for mut transform in q {
        transform.translation.y -= speed * time.delta_secs();
    }
}

fn despawn_zombies(mut commands: Commands, pos: Query<(Entity, &Transform), With<Zombie>>) {
    for (entity, transform) in pos {
        if transform.translation.y < -200.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn linspace(start: f32, end: f32, capacity: usize) -> Vec<f32> {
    match capacity {
        0 => Vec::new(),
        1 => vec![start],
        _ => {
            let increment = (end - start) / (capacity - 1) as f32;

            (0..capacity)
                .map(|i| start + increment * i as f32)
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(linspace(0.0, 10.0, 0), Vec::<f32>::new());
    }

    #[test]
    fn one_element() {
        assert_eq!(linspace(0.0, 10.0, 1), vec![0.0]);
    }

    #[test]
    fn normal_range() {
        assert_eq!(linspace(0.0, 10.0, 5), vec![0.0, 2.5, 5.0, 7.5, 10.0]);
    }

    #[test]
    fn two_elements() {
        assert_eq!(linspace(0.0, 10.0, 2), vec![0.0, 10.0]);
    }

    #[test]
    fn decreasing_range() {
        assert_eq!(linspace(10.0, 0.0, 5), vec![10.0, 7.5, 5.0, 2.5, 0.0]);
    }

    #[test]
    fn negative_range() {
        assert_eq!(linspace(-10.0, 10.0, 5), vec![-10.0, -5.0, 0.0, 5.0, 10.0]);
    }

    #[test]
    fn same_start_and_end() {
        assert_eq!(linspace(5.0, 5.0, 4), vec![5.0, 5.0, 5.0, 5.0]);
    }
}
