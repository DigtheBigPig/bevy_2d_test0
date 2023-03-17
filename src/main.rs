#[allow(dead_code)]
use bevy::prelude::*;

//use bevy_window::prelude::*;

//use bevy_ecs_tilemap::prelude::*;

mod debug;
mod main_menu;
mod helpers;
mod music;
mod gameparts;

use debug::DebugPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "This is the text at the top bar",
                ),
                mode: bevy_window::WindowMode::Windowed,
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        //.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
        .add_startup_system(helpers::camera::camera_setup)
        .add_system(helpers::camera::movement)
        .add_plugin(DebugPlugin)
        .add_system(main_menu::ui_setup.in_schedule(OnEnter(AppState::Menu)))
        .add_system(main_menu::button_system)
        .add_system(music::setup.in_schedule(OnEnter(AppState::InGame)))
        //.add_system(gameparts::tilemap::tilemap)
        .run();
}