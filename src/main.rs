#[allow(dead_code)]
use bevy::prelude::*;

//use bevy_window::prelude::*;

//use bevy_ecs_tilemap::prelude::*;

mod debug;
mod main_menu;
mod helpers;
mod music;
mod gameparts;
mod mouse;
mod ui;

use debug::DebugPlugin;
use mouse::MousePlugin;
use ui::UiPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    Paused,
}

const INSPECT: bool = false;


fn main() {
    App::new()
        // ----------  Initial Setup ----------
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Follow the line game: hold left click to draw",
                ),
                mode: bevy_window::WindowMode::Windowed,
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        //.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
        .add_startup_system(helpers::camera::camera_setup)
        
        // ----------  Always Running ----------
        .add_system(helpers::camera::movement)
        .add_plugin(DebugPlugin)
        .add_plugin(UiPlugin)
        
        
        // ----------  Menu Enter ----------
        //.add_system(main_menu::ui_setup.in_schedule(OnEnter(AppState::Menu)))
        
        // ----------  Menu Exit ----------
        //.add_system(main_menu::button_system)
        
        // ----------  InGame Enter ----------
        .add_system(music::setup.in_schedule(OnEnter(AppState::InGame)))
        .add_system(dummy_function.in_schedule(OnEnter(AppState::InGame)))
        //.add_system(gameparts::tilemap::tilemap)
        
        // ----------  InGame Exit ----------
        .add_plugin(MousePlugin)
        // ----------  Pause Enter ----------
        
        // ----------  Pause Exit ----------
        
        // ----------  Exit Setup ----------
        .run();
}

fn dummy_function() {

}