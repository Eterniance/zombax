use bevy::prelude::*;
use crate::bullet::BULLET_RADIUS;

#[derive(Resource)]
pub struct ZombieAsset {
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct ShooterAsset {
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct BonusAsset {
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct BulletAsset {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AssetInitSet;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, AssetInitSet)
            .add_systems(Startup, setup_assets.in_set(AssetInitSet));
    }
}

fn setup_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let zombie_texture = asset_server.load("zombax1.png");
    let bonus_texture = asset_server.load("bonus_plus_1.png");
    let shooter_texture = asset_server.load("shooter1.png");

    let circle = meshes.add(Circle::new(BULLET_RADIUS));

    let white = materials.add(Color::WHITE);

    commands.insert_resource(ZombieAsset {
        texture: zombie_texture,
    });

    commands.insert_resource(ShooterAsset {
        texture: shooter_texture,
    });

    commands.insert_resource(BonusAsset {
        texture: bonus_texture,
    });

    commands.insert_resource(BulletAsset {
        mesh: circle,
        material: white,
    });
}
