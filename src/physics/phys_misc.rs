use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Collider {
    Solid,
    Thin,
}

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