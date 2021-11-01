use crate::prelude::*;

pub trait Distance<T> {
  fn distance(&self, other: T) -> f32;
}

impl Distance<Vec2> for Vec2 {
  fn distance(&self, other: Vec2) -> f32 {
    return ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt();
  }
}

pub trait ExtendedVec2 {
  fn to_vec3(&self, z: f32) -> Vec3;
}

impl ExtendedVec2 for Vec2 {
  fn to_vec3(&self, z: f32) -> Vec3 {
      return Vec3::new(self.x, self.y, z);
  }
}

pub trait ExtendedVec3 {
  fn to_vec2(&self) -> Vec2;
}

impl ExtendedVec3 for Vec3 {
  fn to_vec2(&self) -> Vec2 {
      return Vec2::new(self.x, self.y);
  }
}
