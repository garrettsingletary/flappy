use crate::states::GameState;
use bevy::prelude::*;


pub struct PlayerPlugin;

const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
pub struct Player {
    pub velocity: f32,
    pub space_pressed: bool,
}

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
            SystemSet::on_enter(GameState::Play)
                .with_system(spawn_player.system()),
            )
            .add_system_set(
            SystemSet::on_exit(GameState::Play)
                .with_system(reset_player.system()),
            );
    }
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
                translation: Vec3::new(50.0, 50.0, 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            velocity: 0.0,
            space_pressed: false,
        });
}

fn reset_player(mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in query.iter_mut() {
        player.velocity = 0.0;
        transform.translation.y = 0.0;
    }
}
