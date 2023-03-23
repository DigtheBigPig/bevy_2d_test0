use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{self, Rng};

use crate::ui::{DifficultyNumber, LengthNumber};

const LINE_WIDTH: f32 = 1.0;

pub struct MousePlugin;  

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GenLineVec>()
            .init_resource::<LineVec>()
            .init_resource::<PrevCursorPos>()
            .init_resource::<CursorPos>()
            .init_resource::<IsPressed>()
            .init_resource::<ScoreNumber>()
            .init_resource::<HighScoreNumber>()
            //.add_system(update_cursor_pos)
            .add_system(mouse_button_input)
            .add_startup_system(generate_line_vec)
            .add_startup_system(draw_box)
            
            
            ;
    }
}

#[derive(Component)]
pub struct DrawingLine;

#[derive(Resource, Debug)]
pub struct PrevCursorPos(Vec2);
impl Default for PrevCursorPos {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

#[derive(Resource, Debug)]
pub struct CursorPos(Vec2);
impl Default for CursorPos {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

#[derive(Resource, Debug)]
pub struct LineVec(pub Vec<Vec2>);
impl Default for LineVec {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec::default())
    }
}


#[derive(Resource, Debug)]
pub struct IsPressed(bool);
impl Default for IsPressed {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(false)
    }
}

pub fn mouse_button_input(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    buttons: Res<Input<MouseButton>>,
    mut line_vec: ResMut<LineVec>,
    mut cur_press_state: ResMut<IsPressed>,
    mut prev_cursor_pos: ResMut<PrevCursorPos>,
    mut cursor_pos: ResMut<CursorPos>,
    mut commands: Commands,
) {
    let is_draw_line = cur_press_state.0 && !is_out_of_bounds(cursor_pos.0) && !is_out_of_bounds(prev_cursor_pos.0);
    for cursor_moved in cursor_moved_events.iter() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *prev_cursor_pos = PrevCursorPos(cursor_pos.0);
                *cursor_pos = CursorPos(pos);
                if is_draw_line {
                    line_vec.0.push(pos);
                }
            }
        }
        break;
    }

    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        *cur_press_state = IsPressed(true);
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
        *cur_press_state = IsPressed(false);
    }

    if is_draw_line {
        commands.spawn((DrawingLine,SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            transform: Transform { 
                translation: (vec3_average(cursor_pos.0.extend(0.0),prev_cursor_pos.0.extend(0.0))), 
                rotation: (Quat::from_rotation_z(vec3_rotation(cursor_pos.0.extend(0.0),prev_cursor_pos.0.extend(0.0)))),
                scale: (Vec3::new(vec3_distance(cursor_pos.0.extend(0.0),prev_cursor_pos.0.extend(0.0)),LINE_WIDTH,1.0)) 
            },
            ..default()
            }));
    }
}

fn vec3_average(u: Vec3, v: Vec3) -> Vec3{
    return Vec3::new((u.x+v.x)/2.0,(u.y+v.y)/2.0,(u.z+v.z)/2.0);
}

fn vec3_distance(u: Vec3, v: Vec3) -> f32{
    let vec = u-v;
    return vec.length();
}

fn vec3_rotation(u: Vec3, v: Vec3) -> f32{
    let vec = u-v;
    if vec.y < 0.0 {
        return -Vec3::angle_between(u-v, Vec3::X);
    }
    else {
        return Vec3::angle_between(u-v, Vec3::X);
    }
    
}

#[derive(Resource, Debug)]
pub struct GenLineVec(pub Vec<Vec2>);
impl Default for GenLineVec {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec::default())
    }
}

const WIDTH_BOX: f32 = 600.0; //half 
const HEIGHT_BOX: f32 = 300.0;
const MAX_CHANGE: f32 = PI/18.0;  // in 0.5 radians if Max_change == 4PI then it can go in any direction
const POINT_DIST: f32 = 2.0;
const NUM_POINTS: usize = 500;

const START_INDEX: usize = 2;
const REMOVE_LIMIT: usize = 50;

pub fn generate_line_vec(
    mut gen_line_vec: ResMut<GenLineVec>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    difficulty: Res<DifficultyNumber>,
    length: Res<LengthNumber>,
) {
    let num_point: usize = NUM_POINTS*(length.0 as usize);
    let max_change: f32 = MAX_CHANGE*difficulty.0;
    gen_line_vec.0.clear();
    let mut rng = rand::thread_rng();
    gen_line_vec.0.push(Vec2::new(-WIDTH_BOX+40.0, HEIGHT_BOX-40.0));
    let prev_point = gen_line_vec.0[0];
    let direction = POINT_DIST*Vec2::new(1.0,0.0);
    gen_line_vec.0.push(generate_next_point(direction, prev_point, rng.gen::<f32>(), max_change));

    //implemnet a while loop so that you can take steps back if you hit the border
    let mut counter: usize = 2;
    let mut iteration_ender: usize = 0;

    

    while counter < num_point {
        let direction = gen_line_vec.0[counter-1]-gen_line_vec.0[counter-2];
        let prev_point = gen_line_vec.0[counter-1];
        if gen_line_vec.0.len()-1 < counter {
            gen_line_vec.0.push(generate_next_point(direction, prev_point, rng.gen::<f32>(), max_change));
        }
        else {
            gen_line_vec.0[counter] = generate_next_point(direction, prev_point, rng.gen::<f32>(), max_change);
        }
        if is_out_of_bounds(gen_line_vec.0[counter]) {
            if iteration_ender>10*num_point {
                break;
            }
            if counter > REMOVE_LIMIT + START_INDEX {
                counter -= REMOVE_LIMIT;
            }
            else {
                counter = START_INDEX
            }
            gen_line_vec.0.drain((counter+1)..);
        }
        counter += 1;
        iteration_ender += 1;
    }
    /*
    for i in 2..NUM_POINTS {
        let direction = gen_line_vec.0[i-1]-gen_line_vec.0[i-2];
        let prev_point = gen_line_vec.0[i-1];
        gen_line_vec.0.push(generate_next_point(direction, prev_point, rng.gen::<f32>()));
    }*/
    draw_gen_line(commands,&gen_line_vec.0, asset_server);
}

fn generate_next_point(direction:Vec2, prev_point:Vec2, rand_num:f32, max_change: f32) -> Vec2 {
    prev_point + rand_rotate(direction,rand_num, max_change)
}

fn rand_rotate(direction:Vec2, rand_num:f32, max_change: f32) -> Vec2 {
    let rand_num_new = (rand_num-0.5)*max_change;
    return Vec2::rotate(Vec2::new(rand_num_new.cos(),rand_num_new.sin()), direction);
}

fn is_out_of_bounds(vec: Vec2) -> bool {
    return !(vec.x > -WIDTH_BOX && vec.x < WIDTH_BOX && vec.y > -HEIGHT_BOX && vec.y < HEIGHT_BOX);
}

#[derive(Component)]
pub struct DrawingGenLine;

fn draw_gen_line(mut commands: Commands, gen_line_vec: &Vec<Vec2>, asset_server: Res<AssetServer>) {
    let vec_length = gen_line_vec.len();
    for i in 1..vec_length {
        let prev_cursor_pos = gen_line_vec[i-1];
        let cursor_pos = gen_line_vec[i];
        let color_grad = (i as f32)/(vec_length as f32);
        let color_grad_r = 1.0-color_grad;
        let color_grad_g = 0.5*(color_grad*20.0).sin()+0.5;
        let color_grad_b = color_grad;
        commands.spawn((DrawingGenLine,SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(color_grad_r.max(1.0), color_grad_g.max(0.0), color_grad_b.max(0.0)),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            transform: Transform { 
                translation: (vec3_average(cursor_pos.extend(0.0),prev_cursor_pos.extend(0.0))), 
                rotation: (Quat::from_rotation_z(vec3_rotation(cursor_pos.extend(0.0),prev_cursor_pos.extend(0.0)))),
                scale: (Vec3::new(vec3_distance(cursor_pos.extend(0.0),prev_cursor_pos.extend(0.0)),LINE_WIDTH,1.0)) 
            },
            ..default()
            }));
    }
    
    // Start marker
    let start_pos = gen_line_vec[0];
    let start_dir = Vec2::new(1.0,0.0);
    const WSIZE:f32 = 50.0;
    const HSIZE:f32 = 25.0;  
    commands.spawn((DrawingGenLine,SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0,1.0,1.0),
            custom_size: Some(Vec2::new(WSIZE, HSIZE)),
            ..default()
        },
        transform: Transform { 
            translation: (start_pos.extend(0.0) + WSIZE/2.0*Vec3::new(-start_dir.y,start_dir.x,0.0)), 
            rotation: (Quat::from_rotation_z(-Vec2::angle_between(start_dir, Vec2::X))),
            scale: (Vec3 { x: 1.0, y: 1.0, z: 1.0 }) 
        },
        ..default()
    }))    
    //add text
    .with_children(|parent| {
        parent.spawn((DrawingGenLine,Text2dBundle{
            transform: Transform::from_translation(Vec3::new(0.0,0.0,0.1)),
            text: Text::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.1, 0.1, 0.1),
                },),
            ..default()
        }
        ));
    });
}

#[derive(Resource, Debug,Default)]
pub struct ScoreNumber(pub f32);

#[derive(Resource, Debug,Default)]
pub struct HighScoreNumber(pub f32);


pub fn compare_line_vecs(
    gen_line_vec: Res<GenLineVec>,
    line_vec: Res<LineVec>,
    mut highscore: ResMut<HighScoreNumber>,
    mut score: ResMut<ScoreNumber>,
    mut text_query: Query<&mut Text, With<crate::ui::ScoreNum>>,
) {

    if line_vec.0.len() == 0 {
        return;
    }
    let mut gen_line_index: usize = 0;
    let mut line_error = 0.0;

    line_error += line_vec.0[0].distance(gen_line_vec.0[0]);
    line_error += line_vec.0[line_vec.0.len()-1].distance(gen_line_vec.0[gen_line_vec.0.len()-1]);

    for line_point in line_vec.0.iter() {
        // find closest point in start
        if gen_line_index < gen_line_vec.0.len()-1 {
            while is_next_closer(*line_point, gen_line_vec.0[gen_line_index], gen_line_vec.0[gen_line_index+1]) {
                gen_line_index += 1;
                if gen_line_index == gen_line_vec.0.len()-1 {
                    gen_line_index -= 1;
                    break;
                }
            }
        }
        line_error += line_point.distance(gen_line_vec.0[gen_line_index]);

    }
    line_error /= line_vec.0.len() as f32 + 2.0;
    score.0 = calculate_score(line_error);
    if score.0 > highscore.0 {
        highscore.0 = score.0;
    }
    let mut text = text_query.get_single_mut().unwrap();
    text.sections[0].value = format!("{:.3}\n{:.3}",score.0.to_string(), highscore.0.to_string());

}

fn is_next_closer(x: Vec2, u: Vec2, v: Vec2) -> bool {
    let u_dist = x.distance(u);
    let v_dist = x.distance(v);
    return u_dist>v_dist;
}

fn calculate_score(e: f32) -> f32 {
    return (100.0-5.0*(e-0.25)).max(0.0).min(100.0);
}


fn draw_box(
    mut commands: Commands,
) {
    //Spawn top border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0,0.0,0.0),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        transform: Transform { 
            translation: (Vec3::new(0.0,HEIGHT_BOX,0.0)), 
            rotation: (default()),
            scale: (Vec3::new(2.0*WIDTH_BOX, 2.0, 0.0)) 
        },
        ..default()
        });
    //Spawn bottom border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0,0.0,0.0),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        transform: Transform { 
            translation: (Vec3::new(0.0,-HEIGHT_BOX,0.0)), 
            rotation: (default()),
            scale: (Vec3::new(2.0*WIDTH_BOX, 2.0, 0.0)) 
        },
        ..default()
        });
    //Spawn left border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0,0.0,0.0),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        transform: Transform { 
            translation: (Vec3::new(-WIDTH_BOX,0.0,0.0)), 
            rotation: (default()),
            scale: (Vec3::new(2.0, 2.0*HEIGHT_BOX, 0.0)) 
        },
        ..default()
        });
    //Spawn right border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0,0.0,0.0),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        transform: Transform { 
            translation: (Vec3::new(WIDTH_BOX,0.0,0.0)), 
            rotation: (default()),
            scale: (Vec3::new(2.0, 2.0*HEIGHT_BOX, 0.0)) 
        },
        ..default()
        });
}