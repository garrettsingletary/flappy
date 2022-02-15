use crate::states::GameState;
use bevy::prelude::*;

mod main_menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(main_menu::setup_main_menu.system()),
        )
        .add_system_set(
            SystemSet::on_resume(GameState::MainMenu)
                .with_system(main_menu::setup_main_menu.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(main_menu::handle_button_click.system()),
        )
        .add_system_set(
            SystemSet::on_pause(GameState::MainMenu)
                .with_system(main_menu::tear_down_menu_items.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::MainMenu)
                .with_system(main_menu::tear_down_menu_items.system()),
        );
    }
}
