mod physics;
mod player;
mod engine;
mod prelude {
    pub use bevy::{
        prelude::*,
        render::pass::ClearColor,
        sprite::collide_aabb::{collide, Collision},
        ecs::component::Component,
        diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}
    };
    pub use crate::physics::*;
    pub use crate::player::*;
    pub use crate::engine::*;
}
use prelude::*;

fn main() {
    
    App::build()
        .add_plugin(PlatformPlayerPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

