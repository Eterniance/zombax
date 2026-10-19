use crate::bullet::BULLET_RADIUS;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ZombieAsset {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
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
    pub sound: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct HealthBarAsset {
    pub mesh: Handle<Mesh>,
    pub health_material: Handle<ColorMaterial>,
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
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let zombie_texture = asset_server.load("zombax.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
    let zombie_layout_handle = texture_atlases.add(layout);

    let bonus_texture = asset_server.load("bonus_plus_1.png");
    let shooter_texture = asset_server.load("shooter1.png");

    let circle = meshes.add(Circle::new(BULLET_RADIUS));

    let rectangle = meshes.add(Rectangle::new(20.0, 2.5));

    let red = materials.add(Color::srgb(1.0, 0.0, 0.1));
    let white = materials.add(Color::WHITE);

    let shoot_sound = asset_server.load("pew.ogg");

    commands.insert_resource(ZombieAsset {
        texture: zombie_texture,
        layout: zombie_layout_handle,
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
        sound: shoot_sound,
    });

    commands.insert_resource( HealthBarAsset {
        mesh: rectangle,
        health_material: red,
    })
}
