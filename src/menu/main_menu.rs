use crate::states::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Clone, Component, Copy)]
pub enum MenuButton {
    Play,
    Quit,
}

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
) {
    let font: Handle<Font> = asset_server.load("fonts/RobotoMono-Regular.ttf");

    // set menu background to black
    clear_color.0 = Color::BLACK;

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            visibility: Visibility {
                is_visible: false,
                ..Visibility::default()
            },
            ..NodeBundle::default()
        })
        .insert(MainMenu)
        .with_children(|mut parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Welcome to Flappy",
                    TextStyle {
                        font: font.clone(),
                        font_size: 50.,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            });
            spawn_button(&mut parent, font.clone(), MenuButton::Play);
            spawn_button(&mut parent, font.clone(), MenuButton::Quit);
        });
}

fn spawn_button(parent: &mut ChildBuilder, font: Handle<Font>, menu_item: MenuButton) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(10.0),
                    height: Val::Px(30.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            ..ButtonBundle::default()
        })
        .insert(menu_item)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style::default(),
                text: Text::with_section(
                    match menu_item {
                        MenuButton::Play => "Play",
                        MenuButton::Quit => "Quit",
                    },
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::DARK_GRAY,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            });
        });
}

pub fn handle_button_click(
    mut exit_event: EventWriter<AppExit>,
    mut state: ResMut<State<GameState>>,
    query: Query<(&Interaction, &MenuButton)>,
) {
    query.for_each(|(interaction, item)| match interaction {
        Interaction::Clicked => match item {
            MenuButton::Play => state
                .push(GameState::Play)
                .map_err(|err| error!("Failed to start game: {}", err))
                .unwrap(),
            MenuButton::Quit => exit_event.send(AppExit),
        },
        Interaction::Hovered => {}
        _ => {}
    });
}

pub fn tear_down_menu_items(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
