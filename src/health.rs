use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: u32,
    pub current: u32,
}

impl Health {
    pub fn new(max: u32) -> Self {
        Self { max, current: max }
    }
}

#[derive(Message)]
pub struct Damage {
    pub target: Entity,
    pub amount: u32,
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Damage>()
            .add_systems(Update, apply_damage.run_if(on_message::<Damage>));
    }
}

fn apply_damage(
    mut commands: Commands,
    mut message: MessageReader<Damage>,
    mut health_query: Query<&mut Health>,
) {
    for Damage { target, amount } in message.read() {
        if let Ok(mut health) = health_query.get_mut(*target) {
            let remaining = health.current.saturating_sub(*amount);
            if remaining == 0 {
                commands.entity(*target).despawn();
            } else {
                health.current = remaining;
            }
        }
    }
}
