use crate::prelude::*;

pub struct Raycast {
  origin: Vec2,
  direction: Vec2,
  collider_types: Vec<ColliderType>,
  collision: Option<RaycastCollision>
}

impl Raycast {
  pub fn new(direction: Vec2) -> Self {
    Raycast { 
      origin: Vec2::ZERO, 
      direction, 
      collider_types: vec!(ColliderType::Solid), 
      collision: None 
    }
  }

  

  pub fn with_origin(mut self, origin: Vec2) -> Self {
    self.origin = origin;
    return self;
  }

  pub fn with_collider_types(mut self, collider_types: Vec<ColliderType>) -> Self {
    self.collider_types = collider_types;
    return self;
  }

  pub fn get_collision(&self) -> Option<RaycastCollision> {
    return self.collision.clone();
  }
}

#[derive(Debug, Clone)]
pub struct RaycastCollision {
  collision_point: Vec2,
  pub entity: Entity,
  pub collider_type: ColliderType
}




pub enum RayAxis {
  X,
  Y
}

pub fn ray_collisions(
  mut rays: Query<(Entity, &mut Raycast)>,
  mut cols: Query<(Entity,&Collider)>
) {
  for(re, mut rc) in rays.iter_mut() {
    rc.collision = None;

    let mut shortest = f32::INFINITY;
    let mut shortest_entity: Option<Entity> = None;

    for(ce, cc) in cols.iter() {
      if ce.id() == re.id() {
        continue;
      }

      if rc.collider_types.iter().any(|&i| i == cc.collider_type) {

        //Find the range within the sprite rec and determine if the ray would pass through it



      }

    }
  }
}
