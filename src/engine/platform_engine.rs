use crate::prelude::*;

fn setup(
    mut coms: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //Create Game Entitties

    //cameras
    coms.spawn_bundle(OrthographicCameraBundle::new_2d());
    coms.spawn_bundle(UiCameraBundle::default()); 
    
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
        sprite: Sprite::new(Vec2::new(600.0, 100.0)),
        material: materials.add(Color::BLACK.into()),
        transform: Transform::from_xyz(150.0, -200.0, 0.0),
        ..Default::default()
    })
    .insert(Position{center: (Vec3::new(150.0, -200.0, 0.0))})
    .insert(Collider::Solid);
    //.insert(MovingPlatform::new(Vec2::new(150.0, -200.0),Vec2::new(250.0, 0.0),0.0));

    coms
    .spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(600.0, 20.0)),
        material: materials.add(Color::BLACK.into()),
        transform: Transform::from_xyz(-150.0, -100.0, 0.0),
        ..Default::default()
    })
    .insert(Position{center:(Vec3::new(-150.0, -100.0, 0.0))})
    .insert(Collider::Thin);

    coms
    .spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(60.0, 280.0)),
        material: materials.add(Color::BLACK.into()),
        transform: Transform::from_xyz(300.0, 20.0, 0.0),
        ..Default::default()
    })
    .insert(Position{center:(Vec3::new(300.0, 20.0, 0.0))})
    .insert(Collider::Solid);

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
            jump_height: 500.0,
            busy: 0.0,
            direction: 0.0
        })
        .insert(Body::new());
}


pub struct PlatformPlayerPlugin;

impl Plugin for PlatformPlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugins(DefaultPlugins)
        .insert_resource(Gravity {force: -500.0})
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(DebugStats {velocity: Vec2::ZERO, position: Vec3::ZERO, is_grounded: true, surface: Some(Collider::Solid)})
        .add_startup_system(setup.system())
        .add_system_set(SystemSet::new()
            .label(PhysicsSystem::UpdateVelocity)
            .with_system(player_movement_system.system())
            .with_system(apply_gravity.system())
            .with_system(body_collision_system.system())
            .with_system(move_platforms.system())
            .with_system(update_position.system())
        )
        .add_system(apply_velocity.system().after(PhysicsSystem::Movement))
        .add_system(update_physics_debug.system())
        .add_system(debug_physics.system())
        .add_system(bevy::input::system::exit_on_esc_system.system());
    }
}
