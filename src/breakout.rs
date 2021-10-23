// use crate::prelude::*;

// pub struct Paddle {
//     speed: f32
// }

// pub struct Ball {
//     velocity: Vec3,
// }

// pub enum Collider {
//     Solid,
//     Scoreable,
//     Paddle,
// }

// pub struct Scoreboard {
//     pub score: usize,
// }

// pub fn setup(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {
//     //Create Game Entitties

//     //cameras
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//     commands.spawn_bundle(UiCameraBundle::default()); 
//     //paddle
//     commands
//         .spawn_bundle(SpriteBundle {
//             material: materials.add(Color::rgb(0.5,0.5,1.0).into()),
//             transform: Transform::from_xyz(0.0, -215.0, 0.0),
//             sprite: Sprite::new(Vec2::new(120.0, 30.0)),
//             ..Default::default()
//         })
//         .insert(Paddle {speed: 500.0})
//         .insert(Collider::Paddle);

//     //ball
//     commands
//         .spawn_bundle(SpriteBundle {
//             material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
//             transform: Transform::from_xyz(0.0, -50.0, 1.0),
//             sprite: Sprite::new(Vec2::new(30.0, 30.0)),
//             ..Default::default()
//         })
//         .insert(Ball {
//             velocity: 200.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
//         });


//     //scoreboard
//     commands.spawn_bundle(TextBundle {
//         text: Text {
//             sections: vec![
//                 TextSection {
//                     value: "Score: ".to_string(),
//                     style: TextStyle {
//                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                         font_size: 40.0,
//                         color: Color::rgb(0.5,0.5,1.0),
//                     },
//                 },
//                 TextSection {
//                     value: "".to_string(),
//                     style: TextStyle {
//                         font: asset_server.load("fonts/FiraMono-Medium.ttf"),
//                         font_size: 40.0,
//                         color: Color::rgb(1.0,0.5,0.5),
//                     },
//                 },
//             ],
//             ..Default::default()
//         },
//         style: Style {
//             position_type: PositionType::Absolute,
//             position: Rect {
//                 top: Val::Px(5.0),
//                 left: Val::Px(5.0),
//                 ..Default::default()
//             },
//             ..Default::default()
//         },
//         ..Default::default()
//     });

//     // Walls
//     let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
//     let wall_thickness = 10.0;
//     let bounds = Vec2::new(900.0,600.0);

//     // left
//     commands
//         .spawn_bundle( SpriteBundle {
//             material: wall_material.clone(),
//             transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
//             sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
//             ..Default::default()
//         })
//         .insert(Collider::Solid);

//     commands
//         .spawn_bundle( SpriteBundle {
//             material: wall_material.clone(),
//             transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
//             sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
//             ..Default::default()
//         })
//         .insert(Collider::Solid);

//     commands
//         .spawn_bundle( SpriteBundle {
//             material: wall_material.clone(),
//             transform: Transform::from_xyz(0.0, -bounds.y, 0.0),
//             sprite: Sprite::new(Vec2::new(bounds.x +wall_thickness, wall_thickness)),
//             ..Default::default()
//         })
//         .insert(Collider::Solid);

//     commands
//         .spawn_bundle( SpriteBundle {
//             material: wall_material.clone(),
//             transform: Transform::from_xyz(0.0, bounds.y, 0.0),
//             sprite: Sprite::new(Vec2::new(bounds.x +wall_thickness, wall_thickness)),
//             ..Default::default()
//         })
//         .insert(Collider::Solid);

//     let brick_rows = 10;
//     let brick_columns = 5;
//     let brick_spacing = 20.0;
//     let brick_size = Vec2::new(150.0, 30.0);
//     let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;

//     let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
//     let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());

//     for row in 0..brick_rows {
//         let y_position = row as f32 * (brick_size.y + brick_spacing);
//         for column in 0..brick_columns {
//             let brick_position = Vec3::new(
//                 column as f32 * (brick_size.x + brick_spacing),
//                 y_position,
//                 0.0,
//             ) + bricks_offset;
//             // Spawn brick
//             commands
//                 .spawn_bundle(SpriteBundle {
//                     material: brick_material.clone(),
//                     sprite: Sprite::new(brick_size),
//                     transform: Transform::from_translation(brick_position),
//                     ..Default::default()
//                 })
//                 .insert(Collider::Scoreable);
//         }
//     }
    
// }

// pub fn paddle_movement_system(
//     time: Res<Time>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&Paddle, &mut Transform)>,
// ) {
//     if let Ok((paddle, mut transform)) = query.single_mut() {
//         let mut direction = 0.0;
//         if keyboard_input.pressed(KeyCode::A) {
//             direction -= 1.0;
//         }

//         if keyboard_input.pressed(KeyCode::D) {
//             direction += 1.0;
//         }

//         let translation = &mut transform.translation;

//         translation.x += direction * paddle.speed * time.delta_seconds();

//         translation.x = translation.x.min(380.0).max(-380.0);
//     }
// }

// pub fn ball_movement_system(
//     time: Res<Time>,
//     mut query: Query<(&Ball, &mut Transform)>,
// ) {
//     let delta_seconds = f32::min(0.2, time.delta_seconds());

//     if let Ok((ball, mut transform)) = query.single_mut() {
//         transform.translation += ball.velocity * delta_seconds;
//     }
// }

// pub fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
//     let mut text = query.single_mut().unwrap();
//     text.sections[0].value = format!("Score: {}", scoreboard.score);
// }

// pub fn ball_collision_system(
//     mut commands: Commands,
//     mut scoreboard: ResMut<Scoreboard>,
//     mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
//     collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
// ) {
//     if let Ok((mut ball, ball_transform, sprite)) = ball_query.single_mut() {
//         let ball_size = sprite.size;
//         let velocity = &mut ball.velocity;

//         // check collision with walls
//         for (collider_entity, collider, transform, sprite) in collider_query.iter() {
//             let collision = collide(
//                 ball_transform.translation,
//                 ball_size,
//                 transform.translation,
//                 sprite.size,
//             );
//             if let Some(collision) = collision {
//                 if let Collider::Scoreable = *collider {
//                     scoreboard.score +=1;
//                     commands.entity(collider_entity).despawn();
//                 }

//                 let mut reflect_x = false;
//                 let mut reflect_y = false;

//                 match collision {
//                     Collision::Left => reflect_x = velocity.x > 0.0,
//                     Collision::Right => reflect_x = velocity.x < 0.0,
//                     Collision::Top => reflect_y = velocity.y < 0.0,
//                     Collision::Bottom => reflect_y = velocity.y > 0.0,
//                 }

//                 if reflect_x {
//                     velocity.x = -velocity.x;
//                 }

//                 if reflect_y {
//                     velocity.y = -velocity.y;
//                 }

//                 if let Collider::Solid = *collider {
//                     break;
//                 }
//             }
//         }
//     }
// }

// pub struct BreakoutPlugin;

// impl Plugin for BreakoutPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app
//         .add_plugins(DefaultPlugins)
//         .insert_resource(Scoreboard {score: 0})
//         .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
//         .add_startup_system(setup.system())
//         .add_system(paddle_movement_system.system())
//         .add_system(ball_collision_system.system())
//         .add_system(ball_movement_system.system())
//         .add_system(scoreboard_system.system())
//         .add_system(bevy::input::system::exit_on_esc_system.system());
//     }
// }