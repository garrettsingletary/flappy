use crate::states::GameState;
use bevy::prelude::*;

pub struct PipesPlugin;
struct PipeTimer(Timer);

#[derive(Component)]
pub struct Pipe {
    pub velocity: f32,
}

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(move_pipe.system())
                .with_system(spawn_pipe.system()),
        )
        .insert_resource(PipeTimer(Timer::from_seconds(2.0, true)));
    }
}

fn spawn_pipe(
    time: Res<Time>,
    mut timer: ResMut<PipeTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("pipe.png"),
                transform: Transform {
                    scale: Vec3::new(5.0, 5.0, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Pipe { velocity: 3.0 });
    }
}

fn move_pipe(mut query: Query<(&mut Pipe, &mut Transform)>) {
    for (pipe, mut transform) in query.iter_mut() {
        transform.translation.x -= pipe.velocity;
    }
}
