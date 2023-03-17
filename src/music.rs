use bevy::prelude::*;


const MUTE: bool = false;
/*
pub fn play_music(
    app_state: Res<State<AppState>>,
    // ...
) {
    match app_state.current() {
        AppState::MainMenu => {
            // TODO: play menu music
        }
        AppState::InGame => {
            // TODO: play game music
        }
        AppState::Paused => {
            // TODO: play pause screen music
        }
    }
}*/


pub fn setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("music/Windless Slopes.ogg");
    if !MUTE {
        audio.play(music);
    }
}