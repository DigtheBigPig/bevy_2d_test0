use bevy::prelude::*;

pub struct UiPlugin;  

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DifficultyNumber>()
            .init_resource::<LengthNumber>()
            .add_startup_system(ui_buttons)
            .add_startup_system(ui_score)
            .add_system(retry_button_system)
            .add_system(generate_new_button_system)
            .add_system(visual_button_system)
            .add_system(getscore_button_system)
            .add_system(difficulty_length_button_system)
            ;
    }
}


const UI_BG: Color = Color::rgb(0.75, 0.75, 0.75);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// components for identifying button types
#[derive(Component)]
pub struct EraseDraw;

#[derive(Component)]
pub struct GenerateNew;

#[derive(Component)]
pub struct GetScore;

#[derive(Component)]
pub struct SetSeed;


fn ui_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    // buttons inside a node
    commands.spawn(NodeBundle {
        style: Style {
            position: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
            size: Size::new(Val::Px(170.0*3.0+20.0), Val::Px(60.0)),
            // horizontally center child text
            justify_content: JustifyContent::Start,
            // vertically center child text
            align_items: AlignItems::Start,
            ..default()
        },
        background_color: UI_BG.into(),
        ..default()
    })    
    .with_children(|parent| {
        //retry button
        parent.spawn((EraseDraw, ButtonBundle {
            style: Style {
                position: UiRect { left: Val::Px(20.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(150.0), Val::Px(55.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        }))
        //add text to button
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Retry",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    
        // new gen line button
        parent.spawn((GenerateNew, ButtonBundle {
            style: Style {
                position: UiRect { left: Val::Px(40.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(150.0), Val::Px(55.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        }))
        //add text to button
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "New line",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
        // get score
        parent
        .spawn((GetScore, ButtonBundle {
            style: Style {
                position: UiRect { left: Val::Px(60.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(150.0), Val::Px(55.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        }))
        //add text to button
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Get score",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
        // set seed line button
        parent
        .spawn((SetSeed, ButtonBundle {
            style: Style {
                position: UiRect { left: Val::Px(80.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(150.0), Val::Px(55.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        }))
        //add text to button
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Set seed",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    });

}


pub fn retry_button_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<EraseDraw>),
    >,
    mut next_state: ResMut<NextState<crate::AppState>>,
    mut remove_line: Query<Entity, With<crate::mouse::DrawingLine>>,
    mut line_vec: ResMut<crate::mouse::LineVec>,
    mut commands: Commands,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                next_state.set(crate::AppState::InGame);
                for entity in &mut remove_line {
                    commands.entity(entity).despawn();
                }
                line_vec.0.clear();
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
}

pub fn generate_new_button_system(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<GenerateNew>),
    >,
    mut remove_line: Query<Entity, With<crate::mouse::DrawingGenLine>>,
    gen_line_vec: ResMut<crate::mouse::GenLineVec>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    difficulty: Res<DifficultyNumber>,
    length: Res<LengthNumber>,
) {
    if let Ok(interaction) = &interaction_query.get_single() {
        match *interaction {
            Interaction::Clicked => {
                for entity in &mut remove_line {
                    commands.entity(entity).despawn();
                }
                crate::mouse::generate_line_vec(gen_line_vec,commands, asset_server, difficulty, length);
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
}

pub fn getscore_button_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<GetScore>),
    >,
    gen_line_vec: Res<crate::mouse::GenLineVec>,
    line_vec: ResMut<crate::mouse::LineVec>,
    highscore: ResMut<crate::mouse::HighScoreNumber>,
    score: ResMut<crate::mouse::ScoreNumber>,
    text_query: Query<&mut Text, With<ScoreNum>>,
) {
    if let Ok(interaction) = &interaction_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                crate::mouse::compare_line_vecs(gen_line_vec, line_vec.into(), highscore, score, text_query);
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
}

pub fn visual_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// add score/ highscore text:

// components for identifying scores types
#[derive(Component)]
pub struct ScoreNum;

#[derive(Component)]
pub struct DifficultyNumM;
#[derive(Component)]
pub struct DifficultyNumP;

#[derive(Component)]
pub struct LengthNumM;
#[derive(Component)]
pub struct LengthNumP;

#[derive(Component)]
pub struct DifficultyLength;

pub fn ui_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<crate::mouse::ScoreNumber>,
    highscore: Res<crate::mouse::HighScoreNumber>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
            size: Size::new(Val::Px(150.0), Val::Px(60.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: UI_BG.into(),
        ..default()
    })    
    //add text
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Score:\nHighScore:",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::rgb(0.1, 0.1, 0.1),
            },
        ));
    })
    .with_children(|parent| {
        parent.spawn((ScoreNum,TextBundle::from_section(
            format!("{}\n{}",score.0.to_string(), highscore.0.to_string()),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::rgb(0.1, 0.1, 0.1),
            },
        )));
    });
    //------------------------------------
    // dificulty choice
    commands.spawn(NodeBundle {
        style: Style {
            position: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
            size: Size::new(Val::Px(250.0), Val::Px(60.0)),
            // horizontally center child text
            justify_content: JustifyContent::Start,
            // vertically center child text
            align_items: AlignItems::Start,
            ..default()
        },
        background_color: UI_BG.into(),
        ..default()
    })    
    .with_children(|parent| {
        // text labels
        parent.spawn(NodeBundle {
            style: Style {
                position: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(10.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(100.0), Val::Px(40.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: UI_BG.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Difficulty:\nLength:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.1, 0.1, 0.1),
                },
            ));
        });
        // first - button
        parent.spawn((DifficultyNumM, ButtonBundle {
            style: Style {
                position: UiRect { left: Val::Px(20.0), right: Val::Px(0.0), top: Val::Px(10.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        }))
        //add text to button
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "-",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
        // second - button
        parent.spawn((LengthNumM, ButtonBundle {
            style: Style {
                position: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(30.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        }))
        //add text to button
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "-",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
        // text in the middle
        parent.spawn( NodeBundle {
            style: Style {
                position: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(10.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(50.0), Val::Px(40.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: UI_BG.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((DifficultyLength,TextBundle::from_section(
                format!("{}\n{}",score.0.to_string(), highscore.0.to_string()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.1, 0.1, 0.1),
                },
            )));
        });
        // plus buttons
        parent
        .spawn((DifficultyNumP, ButtonBundle {
            style: Style {
                position: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(10.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        }))
        //add text to button
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "+",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
        // second plus button
        parent
        .spawn((LengthNumP, ButtonBundle {
            style: Style {
                position: UiRect { left: Val::Px(-20.0), right: Val::Px(0.0), top: Val::Px(30.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        }))
        //add text to button
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "+",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    });

}

#[derive(Resource, Debug)]
pub struct DifficultyNumber(pub f32);
impl Default for DifficultyNumber {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(2.0)
    }
}

#[derive(Resource, Debug)]
pub struct LengthNumber(pub f32);
impl Default for LengthNumber {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(4.0)
    }
}

pub fn difficulty_length_button_system(
    mut interaction_query_dm: Query<
        &Interaction,
        (Changed<Interaction>, With<DifficultyNumM>),
    >,
    mut interaction_query_dp: Query<
        &Interaction,
        (Changed<Interaction>, With<DifficultyNumP>),
    >,
    mut interaction_query_lm: Query<
        &Interaction,
        (Changed<Interaction>, With<LengthNumM>),
    >,
    mut interaction_query_lp: Query<
        &Interaction,
        (Changed<Interaction>, With<LengthNumP>),
    >,
    mut difficulty: ResMut<DifficultyNumber>,
    mut length: ResMut<LengthNumber>,
    mut text_query: Query<&mut Text, With<DifficultyLength>>,
) {
    // Difficulty
    // minus 
    if let Ok(interaction) = &interaction_query_dm.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                difficulty.0 -= 1.0;
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
    // plus
    if let Ok(interaction) = &interaction_query_dp.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                difficulty.0 += 1.0;
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
    // Length
    // minus 
    if let Ok(interaction) = &interaction_query_lm.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                length.0 -= 1.0;
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
    // plus
    if let Ok(interaction) = &interaction_query_lp.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                length.0 += 1.0;
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
    length.0 = length.0.clamp(1.0, 20.0);
    difficulty.0 = difficulty.0.clamp(0.0, 10.0);
    let mut text = text_query.get_single_mut().unwrap();
    text.sections[0].value = format!("{}\n{}",difficulty.0.to_string(), (500.0*length.0).to_string());
}