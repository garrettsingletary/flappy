use bevy::prelude::*;

const WINDOW_HEIGHT: f32 = 500.;
const WINDOW_WIDTH: f32 = 500.;
const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

struct GameOverEvent;

#[derive(Component)]
struct Player {
    velocity: f32, 
    space_pressed: bool,
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
        .insert(Player {
            velocity: 0.0,
            space_pressed: false,
        });
}

fn gravity_and_move(mut game_over_writer: EventWriter<GameOverEvent>, mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in query.iter_mut() {
        if player.velocity < 5.0 {
            player.velocity += 0.2
        }

        let new_pos: f32 = transform.translation.y - player.velocity;

        if new_pos > -WINDOW_WIDTH && new_pos < WINDOW_HEIGHT {
            transform.translation.y -= player.velocity;
        } else {
            game_over_writer.send(GameOverEvent);
        }
    }
}

fn flap(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Player>) {
    for mut player in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) && player.space_pressed == false {
            if player.velocity > -10.0 {
                player.velocity -= 5.0;
            }
            player.space_pressed = true;
        } else if !keyboard_input.pressed(KeyCode::Space) && player.space_pressed {
            player.space_pressed = false;
        }
    }
}

fn game_over(commands: Commands, mut game_over_reader: EventReader<GameOverEvent>) {
    if game_over_reader.iter().next().is_some() {
        println!("GAME OVER");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_system(gravity_and_move)
        .add_system(flap)
        .add_system(game_over)
        .add_event::<GameOverEvent>()
        .run()
}
