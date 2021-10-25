use crate::prelude::*;

pub trait Lerp<T> {
  fn lerp(&self, target: T, time: f32) -> T;
}

pub trait LerpToVec2 {
  fn lerp_to_vec2(&self, target: Vec2, time: f32) -> Vec3;
}

impl Lerp<f32> for f32 {
  fn lerp(&self, target: f32, time: f32) -> f32 {
    return self * (1.0 - time) + target * time;
  }
}

impl Lerp<Vec2> for Vec2 {
    fn lerp(&self, target: Vec2, time: f32) -> Vec2 {
    let s_x = self.x;
    let s_y = self.y;
    let t_x = target.x;
    let t_y = target.y;
    let result_x = s_x * (1.0 - time) + t_x * time;
    let result_y = s_y * (1.0 - time) + t_y * time;
    return Vec2::new(result_x, result_y);
  }
}

impl Lerp<Vec3> for Vec3 {
    fn lerp(&self, target: Vec3, time: f32) -> Vec3 {
    let s_x = self.x;
    let s_y = self.y;
    let s_z = self.z;
    let t_x = target.x;
    let t_y = target.y;
    let t_z = target.z;
    let result_x = s_x * (1.0 - time) + t_x * time;
    let result_y = s_y * (1.0 - time) + t_y * time;
    let result_z = s_z * (1.0 - time) + t_z * time;
    return Vec3::new(result_x, result_y, result_z);
  }
}

impl LerpToVec2 for Vec3 {
    fn lerp_to_vec2(&self, target: Vec2, time: f32) -> Vec3 {
    let s_x = self.x;
    let s_y = self.y;
    let t_x = target.x;
    let t_y = target.y;
    let result_x = s_x * (1.0 - time) + t_x * time;
    let result_y = s_y * (1.0 - time) + t_y * time;
    let result_z = self.z;
    return Vec3::new(result_x, result_y, result_z);
  }
}



