mod states;
use states::GameState;

use bevy::prelude::*;

mod menu;

const WINDOW_HEIGHT: f32 = 1000.;
const WINDOW_WIDTH: f32 = 1000.;
const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
struct Player {
    velocity: f32,
    space_pressed: bool,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
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

fn gravity_and_move(
    mut state: ResMut<State<GameState>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        if player.velocity < 5.0 {
            player.velocity += 0.2
        }

        let new_pos: f32 = transform.translation.y - player.velocity;

        if new_pos > -WINDOW_WIDTH && new_pos < WINDOW_HEIGHT {
            transform.translation.y -= player.velocity;
        } else {
            state.set(GameState::GameOver).unwrap();
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

fn game_over(mut commands: Commands, mut state: ResMut<State<GameState>>, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    state.set(GameState::MainMenu).unwrap();
    println!("GAME OVER");
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "Flappy".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(menu::MenuPlugin)
        .add_startup_system(setup_camera)
        .add_state(states::GameState::MainMenu)
        .add_system_set(SystemSet::on_enter(states::GameState::Play).with_system(spawn_player))
        .add_system_set(
            SystemSet::on_update(states::GameState::Play)
                .with_system(gravity_and_move)
                .with_system(flap),
        )
        .add_system_set(SystemSet::on_enter(states::GameState::GameOver).with_system(game_over))
        .run()
}
