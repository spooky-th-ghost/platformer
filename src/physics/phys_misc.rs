use crate::prelude::*;

pub struct Gravity {
    pub force: f32,
}

pub struct Position{pub center: Vec3}

pub struct GeoLine {
    pub start: Vec2,
    pub end: Vec2
}

impl GeoLine {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        GeoLine {
            start,
            end
        }
    }

    pub fn intersects(&self, other: GeoLine) -> bool{
        let X1 = self.start.x;
        let X2 = self.end.x;
        let Y1 = self.start.y;
        let Y2 = self.end.y;
        let X3 = other.start.x;
        let X4 = other.end.x;
        let Y3 = other.start.y;
        let Y4 = other.end.y;
        let dx0 = X2-X1;
        let dx1 = X4-X3;
        let dy0 = Y2-Y1;
        let dy1 = Y4-Y3;
        let p0 = dy1*(X4-X1) - dx1*(Y4-Y1);
        let p1 = dy1*(X4-X2) - dx1*(Y4-Y2);
        let p2 = dy0*(X2-X3) - dx0*(Y2-Y3);
        let p3 = dy0*(X2-X4) - dx0*(Y2-Y4);
        return (p0*p1<=0.0) & (p2*p3<=0.0);
    }
}
pub struct FloatRange {
  min: f32,
  max: f32,
}

impl FloatRange {
  pub fn new(min: f32, max: f32) -> Self {
    FloatRange {
      min,
      max
    }
  }

  pub fn contains(&self, value: f32) -> bool {
    return self.min >= value && self.max <= value;
  }
}
pub struct DebugStats {
    pub velocity: Vec2,
    pub position: Vec3,
    pub is_grounded: bool,
    pub surface: Option<ColliderType>,
}

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
