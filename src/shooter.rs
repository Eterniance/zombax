use crate::assets::{AssetInitSet, ShooterAsset};
use bevy::prelude::*;

#[rustfmt::skip]
const FORMATION: &[Vec2] = &[
                    vec2(0.0, 80.0),
        vec2(-48.0, 60.0),      vec2(48.0, 60.0),    
    vec2(-60.0, 0.0),               vec2(60.0, 0.0),
        vec2(-48.0, -60.0),     vec2(48.0, -60.0),
                    vec2(0.0, -80.0),
];

#[derive(Component)]
pub struct MainShooter;

#[derive(Component)]
pub struct Shooter;

#[derive(Message)]
pub struct SpawnShooter;

pub struct ShooterPlugin;

impl Plugin for ShooterPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnShooter>()
            .add_systems(Startup, spawn_main_shooter.after(AssetInitSet))
            .add_systems(
                Update,
                (
                    move_shooter,
                    spawn_shooter.run_if(on_message::<SpawnShooter>),
                ),
            );
    }
}

fn spawn_main_shooter(mut commands: Commands, shooter_asset: Res<ShooterAsset>) {
    commands.spawn((
        MainShooter,
        Shooter,
        Mesh2d(shooter_asset.mesh.clone()),
        MeshMaterial2d(shooter_asset.material.clone()),
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));
}

fn spawn_shooter(
    mut commands: Commands,
    main_shooter_q: Single<&Transform, With<MainShooter>>,
    shooters_q: Query<(), With<Shooter>>,
    shooter_asset: Res<ShooterAsset>,
) {
    let shooters_count = shooters_q.iter().count();
    let center_position = main_shooter_q.translation;

    if let Some(offset) = FORMATION.get(shooters_count - 1) {
        let world_pos = center_position + offset.extend(0.0);

        commands.spawn((
            Shooter,
            Mesh2d(shooter_asset.mesh.clone()),
            MeshMaterial2d(shooter_asset.material.clone()),
            Transform::from_translation(world_pos),
        ));
    } else {
        // commands.spawn(Shooter);
        info!("No more shooter for now !");
    }
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
