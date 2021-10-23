use crate::prelude::*;

pub struct Player {
    speed: f32,
    jump_height: f32,
}

pub struct Body {
    velocity: Vec2,
    is_grounded: bool,
    left_wall: bool,
    right_wall: bool,
    surface: Option<Collider>,
}

#[derive(Debug, Clone, Copy)]
pub enum Collider {
    Solid,
    Thin,
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
}

pub struct Gravity {
    force: f32,
}

pub struct Position{center: Vec3}

pub struct DebugStats {
    pub velocity: Vec2,
    pub position: Vec3,
    pub is_grounded: bool,
    pub surface: Option<Collider>,
}

fn setup(
    mut coms: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //Create Game Entitties

    //cameras
    coms.spawn_bundle(OrthographicCameraBundle::new_2d());
    coms.spawn_bundle(UiCameraBundle::default()); 
    //player
    coms
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, -0.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 60.0)),
            ..Default::default()
        })
        .insert(Player {
            speed: 500.0,
            jump_height: 500.0
        })
        .insert(Body::new());
    
    coms.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Velocity: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5,0.5,1.0),
                    },
                },
                TextSection {
                    value: "Position ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0,0.5,0.5),
                    }
                },
                TextSection {
                    value: "Is Grounded ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0,0.5,0.5),
                    }
                },

            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    coms
    .spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(600.0, 10.0)),
        material: materials.add(Color::BLACK.into()),
        transform: Transform::from_xyz(150.0, -200.0, 0.0),
        ..Default::default()
    })
    .insert(Position{center: (Vec3::new(150.0, -200.0, 0.0))})
    .insert(Collider::Thin);

    coms
    .spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(600.0, 30.0)),
        material: materials.add(Color::BLACK.into()),
        transform: Transform::from_xyz(-150.0, 0.0, 0.0),
        ..Default::default()
    })
    .insert(Position{center:(Vec3::new(-150.0, 0.0, 0.0))})
    .insert(Collider::Solid);

    coms
    .spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(30.0, 600.0)),
        material: materials.add(Color::BLACK.into()),
        transform: Transform::from_xyz(20.0, 0.0, 0.0),
        ..Default::default()
    })
    .insert(Position{center:(Vec3::new(20.0, 0.0, 0.0))})
    .insert(Collider::Solid);
}

pub fn body_collision_system(
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
            let collision = collide(
                body_transform.translation,
                body_sprite.size,
                collider_position.center,
                collider_sprite.size,
            );
            if let Some(collision) = &collision {
                match collision {
                    Collision::Top => {
                        body.is_grounded = true;
                        body.velocity.y = 0.0;
                        body.surface = Some(collider_type.clone());
                        body_transform.translation.y += 1.0;
                    },
                    Collision::Bottom => {
                        body.velocity.y = 0.0;
                        body_transform.translation.y -= 1.0;
                    },
                    Collision::Left => {
                        body.right_wall = true;
                        body_transform.translation.x -=1.0;
                    },
                    Collision::Right => {
                        body.left_wall = true;
                        body_transform.translation.x +=1.0;
                    },
                }
            }
        }
    }
}


// for col in collisions {
//     if matches!(col,Collision::Top) {
//         body.is_grounded = true;
//         body.velocity.y = 0.0;
//     } else {
//         body.is_grounded = false;
//     }

//     if matches!(col,Collision::Bottom) {
//         body.velocity.y = 0.0;
//     }

//     if matches!(col, Collision::Left) {
//         body.left_wall = true;
//     } else {
//         body.left_wall = false;
//     }

//     if matches!(col, Collision::Right) {
//         body.right_wall = true;
//     } else {
//         body.right_wall = false;
//     }
// }

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

pub fn apply_physics(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Body, &mut Transform)>,
) {
    for(mut body, mut transform) in query.iter_mut() {
        body.apply_gravity(&time, &gravity);
        body.apply_velocity(&mut transform, &time);
    }
}



pub fn debug_physics(stats: Res<DebugStats>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut().unwrap();
    text.sections[1].value = format!("Velocity: {}", stats.velocity);
    text.sections[2].value = format!("Position: {}", stats.position);
    text.sections[0].value = format!("Is Grounded: {:?}", stats.surface);

}

pub fn update_physics_debug(mut stats: ResMut<DebugStats>, query: Query<(&Body, &Transform), With<Player>>) {
    if let Ok((body, transform)) = query.single() {
        stats.velocity = body.velocity;
        stats.surface = body.surface;
    }
}


pub struct PlatformPlayerPlugin;

impl Plugin for PlatformPlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugins(DefaultPlugins)
        .add_plugin(Physics2dPlugin)
        .insert_resource(Gravity {force: -500.0})
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(DebugStats {velocity: Vec2::ZERO, position: Vec3::ZERO, is_grounded: true, surface: Some(Collider::Solid)})
        .add_startup_system(setup.system())
        .add_system(player_movement_system.system())
        .add_system(body_collision_system.system())
        .add_system(apply_physics.system())
        .add_system(update_physics_debug.system())
        .add_system(debug_physics.system())
        .add_system(bevy::input::system::exit_on_esc_system.system());
    }
}