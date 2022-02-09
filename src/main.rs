use bevy::prelude::*;

#[derive(Component)]
struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run()
}

