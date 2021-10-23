mod breakout;
mod platform_engine;
mod prelude {
    pub use bevy::{
        prelude::*,
        render::pass::ClearColor,
        sprite::collide_aabb::{collide, Collision},
    };
    pub use bevy_physimple::prelude::*;
    pub use crate::breakout::*;
    pub use crate::platform_engine::*;
}
use prelude::*;

fn main() {
    App::build()
        .add_plugin(PlatformPlayerPlugin)
        .run();
}

