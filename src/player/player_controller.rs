use crate::prelude::*;

pub struct Player {
    pub speed: f32,
    pub jump_height: f32,
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Body, &mut Transform)>,
) {
    if let Ok((player, mut body, mut transform)) = query.single_mut() {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::A) && !body.left_wall{
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) && !body.right_wall {
            direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            if body.is_grounded {
                if let Some(surface)  = body.surface {
                    match surface {
                        Collider::Thin => {
                            transform.translation.y -= 20.0;
                            body.is_grounded = false;
                            body.velocity.y = -player.jump_height / 2.0;
                        },
                        _ => (),
                    }
                }
            }
        }

        if keyboard_input.pressed(KeyCode::Space) {
            if body.is_grounded {
                transform.translation.y += 5.0;
                body.is_grounded = false;
                body.velocity.y = player.jump_height;
            }
            
        }

        body.velocity.x = direction * player.speed * time.delta_seconds();
        let translation = &mut transform.translation;
        translation.x += body.velocity.x.min(380.0).max(-380.0);
    }
}