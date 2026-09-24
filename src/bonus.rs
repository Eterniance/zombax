use crate::{
    assets::BonusAsset,
    collisions::{HitBox, collides},
    shooter::{MainShooter, SpawnShooter},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Bonus;

#[derive(Resource)]
pub struct SpawnBonusTimer(Timer);

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnBonusTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (move_bonus, spawn_bonus, despawn_bonus, collisions));
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
            Sprite {
                image: bonus_asset.texture.clone(),
                ..default()
            },
            Transform::from_xyz(400.0, 400.0, 0.0),
            HitBox::Box {
                length: 50.0,
                width: 50.0,
            },
        ));
    }
}

fn move_bonus(time: Res<Time>, q: Query<&mut Transform, With<Bonus>>) {
    let speed = 150.0;
    for mut transform in q {
        transform.translation.y -= speed * time.delta_secs();
        transform.translation.z = -transform.translation.y;
    }
}

fn collisions(
    mut commands: Commands,
    bonus_query: Query<(Entity, &Transform, &HitBox), With<Bonus>>,
    shooter_query: Query<(&Transform, &HitBox), With<MainShooter>>,
    mut message_writer: MessageWriter<SpawnShooter>,
) {
    for (bonus_entity, bonus_transform, bonus_hitbox) in &bonus_query {
        for (shooter_transform, shooter_hitbox) in &shooter_query {
            if collides(
                bonus_transform.translation.truncate(),
                bonus_hitbox,
                shooter_transform.translation.truncate(),
                shooter_hitbox,
            ) {
                message_writer.write(SpawnShooter);
                commands.entity(bonus_entity).despawn();
                break;
            }
        }
    }
}

fn despawn_bonus(mut commands: Commands, pos: Query<(Entity, &Transform), With<Bonus>>) {
    for (entity, transform) in pos {
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn();
        }
    }
}
