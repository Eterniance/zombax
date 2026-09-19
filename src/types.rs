use bevy::{prelude::*, ecs::component::Component};

#[derive(Component)]
pub struct Shooter;

#[derive(Component)]
pub struct Zombie;

#[derive(Component)]
pub struct Bonus;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);