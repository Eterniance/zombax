use bevy::prelude::*;

#[derive(Resource)]
pub struct ZombieAsset {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AssetInitSet;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, AssetInitSet)
            .add_systems(Startup, setup_assets.in_set(AssetInitSet));
    }
}

pub fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(50.0, 50.0));

    // Materials
    let red = materials.add(Color::srgb(1.0, 0.0, 0.0));
    let green = materials.add(Color::srgb(0.0, 1.0, 0.0));
    let blue = materials.add(Color::srgb(0.0, 0.0, 1.0));

    commands.insert_resource(ZombieAsset {
        mesh: mesh.clone(),
        material: green,
    });

    commands.insert_resource(ShooterAsset {
        mesh: mesh.clone(),
        material: red,
    });

    commands.insert_resource(BonusAsset {
        mesh,
        material: blue,
    });
}
