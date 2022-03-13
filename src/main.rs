mod menu;
mod pipes;
mod player;
mod states;

use bevy::prelude::*;
use menu::MenuPlugin;
use pipes::*;
use player::*;
use states::GameState;

const WINDOW_HEIGHT: f32 = 1000.;
const WINDOW_WIDTH: f32 = 1000.;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
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

        if new_pos > -WINDOW_HEIGHT / 2.0 && new_pos < WINDOW_HEIGHT / 2.0 {
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

fn game_over(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    state.set(GameState::MainMenu).unwrap();
}

fn move_pipe(mut query: Query<(&mut Pipe, &mut Transform)>) {
    for (pipe, mut transform) in query.iter_mut() {
        transform.translation.x -= pipe.velocity;
    }
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
        .add_plugin(MenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PipesPlugin)
        .add_startup_system(setup_camera)
        .add_state(GameState::MainMenu)
        .add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(gravity_and_move)
                .with_system(flap)
                .with_system(move_pipe)
        )
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(game_over))
        .run()
}
