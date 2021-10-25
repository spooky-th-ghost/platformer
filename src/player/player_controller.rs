use crate::prelude::*;

pub struct Player {
    pub speed: f32,
    pub jump_height: f32,
    pub busy: f32,
    pub direction: f32,
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Body, &mut Transform)>,
) {
    if let Ok((mut player, mut body, mut transform)) = query.single_mut() {
        player.busy -=  time.delta_seconds();
        player.busy = player.busy.clamp(0.0,5.0);
        if player.busy == 0.0 {player.direction = 0.0}

        
        if keyboard_input.pressed(KeyCode::A) && !body.left_wall{
            if player.busy == 0.0 {
                player.direction -= 1.0;
            }
             if body.left_wall {body.velocity.x = body.velocity.x.clamp(0.0,500.0)}
        }

        if keyboard_input.pressed(KeyCode::D) {
            if player.busy == 0.0 {
                player.direction += 1.0;
            }

            if body.right_wall {body.velocity.x = body.velocity.x.clamp(-500.0,0.0)}
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

        if keyboard_input.just_pressed(KeyCode::Space) {
            if body.is_grounded {
                transform.translation.y += 5.0;
                body.is_grounded = false;
                body.velocity.y = player.jump_height;
            } else {
                if body.right_wall && player.busy == 0.0{
                    body.velocity.y = player.jump_height;
                    player.direction = -1.0;
                    player.busy = 0.175;
                }

                if body.left_wall && player.busy == 0.0 {
                    body.velocity.y = player.jump_height;
                    player.direction = 1.0;
                    player.busy = 0.175;
                }
            }

            
        }

        body.velocity.x = player.direction * player.speed * time.delta_seconds();
        let translation = &mut transform.translation;
        translation.x += body.velocity.x.min(380.0).max(-380.0);
    }
}
