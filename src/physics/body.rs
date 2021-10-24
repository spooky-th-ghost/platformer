use crate::prelude::*;
#[derive(Debug)]
pub struct Body {
    pub velocity: Vec2,
    pub is_grounded: bool,
    pub left_wall: bool,
    pub right_wall: bool,
    pub surface: Option<Collider>,
}

impl Body {
    pub fn new() -> Self {
        Body { 
            velocity: Vec2::ZERO,
            is_grounded: true,
            left_wall: false,
            right_wall: false,
            surface: None,
        }
    }

    pub fn apply_gravity(self: &mut Self, time: &Time, gravity: &Gravity) {
        if !self.is_grounded {
            let acceleration = gravity.force;
            self.velocity.y += time.delta_seconds() * acceleration;
            // translation.y += time.delta_seconds() * (self.velocity.y + time.delta_seconds() * acceleration / 2.0);
        }
        
    }

    pub fn apply_velocity(self: &mut Self, transform: &mut Transform, time: &Time) {
        transform.translation += time.delta_seconds() * Vec3::new(self.velocity.x, self.velocity.y, 0.0);
    }
    
    pub fn move_to_collision(self: &mut Self, transform: &mut Transform, sprite: &Sprite, collider: &Collider, collider_pos: &Position, collider_sprite: &Sprite, collision: &Collision) {
        let calc_true_offset = |s_size: f32, c_size: f32, c_pos: f32, s_dir:i8| -> f32 {
            let s_offset = s_size / 2.0;
            let c_offset = c_size / 2.0;
            let s_edge = s_dir as f32* (s_offset);
            let c_edge = s_dir as f32 * (c_offset);
            return c_pos+(c_edge+s_edge);
        };

        match collider {
            Collider::Thin => {
                match collision  {
                    Collision::Top => {
                        let offset = calc_true_offset(sprite.size.y, collider_sprite.size.y, collider_pos.center.y, 1);
                        transform.translation.y = offset;
                        self.surface = Some(collider.clone());
                        self.is_grounded = true;
                    },
                    _ => ()
                }
            },
            Collider::Solid => {
                match collision  {
                    Collision::Bottom => {
                        let new_center = calc_true_offset(sprite.size.y, collider_sprite.size.y, collider_pos.center.y, -1);
                        transform.translation.y = new_center;
                        self.velocity.y = 0.0;
                    },
                    Collision::Top => {
                        let new_center = calc_true_offset(sprite.size.y, collider_sprite.size.y, collider_pos.center.y, 1);
                        transform.translation.y = new_center;
                        self.velocity.y = 0.0;
                        self.surface = Some(collider.clone());
                        self.is_grounded = true;
                    },
                    Collision::Left => {
                        let new_center = calc_true_offset(sprite.size.x, collider_sprite.size.x, collider_pos.center.x, -1);
                        transform.translation.x = new_center;
                        self.velocity.x = 0.0;
                        self.right_wall = true;
                    },
                    Collision::Right => {
                        let new_center = calc_true_offset(sprite.size.x, collider_sprite.size.x, collider_pos.center.x, 1);
                        transform.translation.x = new_center;
                        self.velocity.x = 0.0;
                        self.right_wall = true;
                    }
                }
            }
        }
    }
}

pub fn body_collision_system(
    time: Res<Time>,
    mut body_query: Query<(&mut Body, &mut Transform, &Sprite)>,
    collider_query: Query<(&Position, &Sprite, &Collider)>,
) {
    for(mut body, mut body_transform, body_sprite) in body_query.iter_mut() {
        body.is_grounded = false;
        body.left_wall = false;
        body.right_wall = false;
        body.surface = None;
        // check collision with walls
        for (collider_position, collider_sprite, collider_type) in collider_query.iter() {
            let projected_pos= body_transform.translation + time.delta_seconds() * Vec3::new(body.velocity.x, body.velocity.y, 0.0);
            let collision = collide(
                projected_pos,
                body_sprite.size,
                collider_position.center,
                collider_sprite.size,
            );
            if let Some(collision) = &collision {
                body.move_to_collision(&mut body_transform, body_sprite, collider_type, collider_position, collider_sprite, collision);
            }
        }
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Body, &mut Transform)>,
) {
    for(mut body, mut transform) in query.iter_mut() {
        body.apply_gravity(&time, &gravity);
    }
}

pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Body, &mut Transform)>,
) {
    for(mut body, mut transform) in query.iter_mut() {
        body.apply_velocity(&mut transform, &time);
    }
}