//! This example illustrates the various features of Bevy UI.

use bevy::{
    prelude::*,
    winit::WinitSettings,
};

mod debug;

use debug::DebugPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(camera_setup)
        //.add_startup_system(setup_button)
        .add_startup_system(ui_setup)
        .add_plugin(DebugPlugin)
        .add_system(button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    

    // root node
    commands
        // Main background and root ui element
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        // Add gray rectangle child of root
        .with_children(|parent|{
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(500.0), Val::Px(75.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        position: UiRect::new(Val::Px(150.0), Val::Px(250.0), Val::Px(250.0), Val::Px(50.0)),
                        gap: Size::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                // Add button as child of grey rectangle
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        //add text to button
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Options",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        //add text to button
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "New game",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
        
}


// Button



fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Pressed".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "New game".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "New game".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}