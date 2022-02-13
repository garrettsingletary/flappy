use bevy::prelude::*;

const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Bundle)]
struct Player {
    position: Position,
    velocity: Velocity,
}

#[derive(Component)]
struct Velocity {
    velocity: f32,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(Player {
            position: Position { x: 5, y: 5 },
            velocity: Velocity { velocity: 0.0 },
        });
}

fn gravity_and_move(mut query: Query<(&mut Velocity, &mut Transform)>) {
    for (mut velocity, mut transform) in query.iter_mut() {
        if velocity.velocity < 5.0 {
            velocity.velocity += 0.2
        }

        transform.translation.y -= velocity.velocity;
    }
}

fn flap(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            velocity.velocity -= 1.0;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_system(gravity_and_move)
        .add_system(flap.system())
        .run()
}
