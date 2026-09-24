use crate::{
    assets::{AssetInitSet, ShooterAsset},
    collisions::HitBox,
};
use bevy::prelude::*;

#[rustfmt::skip]
const FORMATION: &[Vec3] = &[
                    vec3(  0.0,  55.0, -55.0),

             vec3(-30.0,  35.0, -35.0), vec3(30.0,  35.0, -35.0),

        vec3(-45.0,  15.0, -15.0), vec3(-15.0,  18.0, -18.0),
        vec3( 15.0,  18.0, -18.0), vec3( 45.0,  15.0, -15.0),

    vec3(-60.0, -15.0,  15.0), vec3(-30.0, -12.0,  12.0),
    vec3( 30.0, -12.0,  12.0), vec3( 60.0, -15.0,  15.0),

        vec3(-45.0, -40.0,  40.0), vec3(-15.0, -43.0,  43.0),
        vec3( 15.0, -43.0,  43.0), vec3( 45.0, -40.0,  40.0),

             vec3(-30.0, -65.0,  65.0), vec3(30.0, -65.0,  65.0),

                    vec3(0.0, -90.0, 90.0),
];

const SHOOTER_SCALE: f32 = 65.0 / 32.0;
const SHOOTER_HITBOX: HitBox = HitBox::Box {
    length: 50.0,
    width: 50.0,
};

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
        Sprite {
            image: shooter_asset.texture.clone(),
            ..Default::default()
        },
        Transform {
            translation: vec3(0.0, -200.0, 0.0),
            scale: Vec3::splat(SHOOTER_SCALE),
            ..Default::default()
        },
        SHOOTER_HITBOX,
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
        let world_pos = center_position + offset;

        commands.spawn((
            Shooter,
            Sprite {
                image: shooter_asset.texture.clone(),
                ..Default::default()
            },
            Transform {
                translation: world_pos,
                scale: Vec3::splat(SHOOTER_SCALE),
                ..Default::default()
            },
            SHOOTER_HITBOX,
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
