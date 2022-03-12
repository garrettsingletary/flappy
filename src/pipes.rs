use crate::states::GameState;
use bevy::prelude::*;


pub struct PipesPlugin;

impl Plugin for PipesPlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Play)
                .with_system(spawn_pipe.system()),
        );
    }
}

fn spawn_pipe(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("pipe.png"),
        ..Default::default()
    });
}


