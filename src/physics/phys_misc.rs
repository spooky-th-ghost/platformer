use crate::prelude::*;

pub struct Gravity {
    pub force: f32,
}

pub struct Position{pub center: Vec3}

pub struct DebugStats {
    pub velocity: Vec2,
    pub position: Vec3,
    pub is_grounded: bool,
    pub surface: Option<Collider>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PhysicsSystem {
    UpdateVelocity,
    Movement,
}

pub fn update_position (mut query: Query<(&mut Position, &Transform)>) {
    for (mut pos, transform) in query.iter_mut() {
        pos.center = transform.translation;
    }
}

pub fn debug_physics(stats: Res<DebugStats>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut().unwrap();
    text.sections[1].value = format!("Velocity: {}", stats.velocity);
    text.sections[2].value = format!("Position: {}", stats.position);
    text.sections[0].value = format!("Is Grounded: {:?}", stats.surface);

}

pub fn update_physics_debug(mut stats: ResMut<DebugStats>, query: Query<(&Transform, &Body), With<Player>>) {
    if let Ok((transform, body)) = query.single() {
        stats.position = transform.translation;
        stats.surface = body.surface;
    }
}
