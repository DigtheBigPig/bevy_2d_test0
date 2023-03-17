use bevy::prelude::*;


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    

    // root node
    commands
        // Main background and root ui element
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server.load("branding/icon.png").into(),
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(250.0)),
                        position: UiRect::new(Val::Px(150.0), Val::Px(10.0), Val::Px(250.0), Val::Px(50.0)),
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



pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<crate::AppState>>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Pressed".to_string();
                *color = PRESSED_BUTTON.into();
                next_state.set(crate::AppState::InGame);
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