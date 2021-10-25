use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Collider {
    Solid,
    Thin,
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
      transform.translation = transform.translation.lerp_to_vec2(platform.end,time.delta_seconds());
    }
  }

  
