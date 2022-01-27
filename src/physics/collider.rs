use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColliderType {
    Solid,
    Thin,
}


pub struct Collider {
  pub collider_type: ColliderType,
  pub position: Vec2,
  pub size: Vec2,
}

impl Collider {
  pub fn new(collider_type: ColliderType, position: Vec2, size: Vec2) -> Self {
    Collider {
      collider_type,
      position,
      size,
    }
  }

  pub fn edges(&self) -> Vec<GeoLine> {
    let mut my_edges: Vec<GeoLine> = Vec::new();
    let corner_1 = Vec2::new(self.position.x - (self.size.x / 2.0), self.position.y + (self.size.y / 2.0));
    let corner_2 = Vec2::new(self.position.x + (self.size.x / 2.0), self.position.y + (self.size.y / 2.0));
    let corner_3 = Vec2::new(self.position.x + (self.size.x / 2.0), self.position.y - (self.size.y / 2.0));
    let corner_4 = Vec2::new(self.position.x - (self.size.x / 2.0), self.position.y - (self.size.y / 2.0));
    
    my_edges.push(GeoLine::new(corner_1,corner_2));
    my_edges.push(GeoLine::new(corner_2,corner_3));
    my_edges.push(GeoLine::new(corner_3,corner_4));
    my_edges.push(GeoLine::new(corner_4,corner_1));
    return my_edges;
  }
}

pub struct MovingPlatform {
  start: Vec2,
  end: Vec2,
  wait_time: f32,
  timer: f32,
  to_end: bool
}

impl MovingPlatform {
  pub fn new(start: Vec2, end: Vec2, wait_time: f32) -> Self {
    MovingPlatform {
      start,
      end,
      wait_time,
      timer: 0.0,
      to_end: true
    }
  }
}


  pub fn move_platforms(
    time: Res<Time>,
    mut query: Query<(&mut MovingPlatform, &mut Transform)>,
  ) {
    for (mut platform, mut transform) in query.iter_mut() {
      transform.translation = transform.translation.lerp(platform.end.to_vec3(transform.translation.z),time.delta_seconds());
    }
  }

  
