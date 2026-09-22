use bevy::prelude::*;

#[derive(Resource)]
pub struct ZombieAsset {
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct ShooterAsset {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct BonusAsset {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
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

    let square = meshes.add(Rectangle::new(50.0, 50.0));
    let circle = meshes.add(Circle::new(5.0));

    let red = materials.add(Color::srgb(1.0, 0.0, 0.0));
    let blue = materials.add(Color::srgb(0.0, 0.0, 1.0));
    let white = materials.add(Color::WHITE);

    commands.insert_resource(ZombieAsset {
        texture: zombie_texture,
    });

    commands.insert_resource(ShooterAsset {
        mesh: square.clone(),
        material: red,
    });

    commands.insert_resource(BonusAsset {
        mesh: square,
        material: blue,
    });

    commands.insert_resource(BulletAsset {
        mesh: circle,
        material: white,
    });
}
