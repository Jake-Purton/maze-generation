use bevy::{prelude::*, utils::HashSet};

use crate::{AppState, setup::StartEnd, MAP_SIZE, MazeMapId};

pub struct AStarPlugin;

impl Plugin for AStarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set (
            SystemSet::on_enter(AppState::AStar)
                .with_system(setup)
        )
        .add_system_set (
            SystemSet::on_update(AppState::AStar)
                .with_system(a_star)
        )
        .add_system_set (
            SystemSet::on_exit(AppState::AStar)
        );
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct AStarCell {
    coordinate: (usize, usize),
    f: usize,
    g: usize,
    came_from: Option<(usize, usize)>
}

#[derive(Resource)]
pub struct SearchedCellsVec (Vec<AStarCell>);

#[derive(Resource)]
pub struct SearchedCellsSet (HashSet<(usize, usize)>);

fn setup (
    mut commands: Commands,
    start_end: Res<StartEnd>,
) {
    let mut set = HashSet::new();
    let mut vec = Vec::new();

    let start = start_end.start.unwrap();
    let end = start_end.end.unwrap();
    let h = distance_between_indexes(start, end);
    let cell = AStarCell { coordinate: start, f: h, g: 0, came_from: None };

    vec.push(cell);
    set.insert(start);

    commands.insert_resource(SearchedCellsVec(vec));
    commands.insert_resource(SearchedCellsSet(set));
}

// each node has
// G cost -> distance from start
// H cost -> distance from end
// F cost -> G + H
// The node it came from (in order to find the path it took)

fn a_star (
    start_end: Res<StartEnd>,
    mut searched_vec: ResMut<SearchedCellsVec>,
    // buttons: Res<Input<MouseButton>>,
    mut images: ResMut<Assets<Image>>,
    id: Res<MazeMapId>,
    mut searched_set: ResMut<SearchedCellsSet>,
    mut app_state: ResMut<State<AppState>>,
) {
    
    // if buttons.just_pressed(MouseButton::Left) {

        let handle = Handle::weak(id.0);

        if let Some(image) = images.get_mut(&handle) {

        searched_vec.0.sort_by_key(|a| -(a.f as i32));
    
        let lowest_f_cost = searched_vec.0.pop().unwrap();
        let new_cells = find_neighbors( lowest_f_cost.coordinate, (MAP_SIZE * 8).try_into().unwrap());

            for i in new_cells {

                let mut index = i.0 * 4 + (i.1 * MAP_SIZE * 8 * 4);
                let h = distance_between_indexes(i, start_end.end.unwrap());

                if image.data[index] == 255 
                && image.data[index + 1] == 255 
                && image.data[index + 2] == 255 
                && image.data[index + 3] == 255 
                {
                    continue;
                }

                let g = distance_between_indexes(i, lowest_f_cost.coordinate) + lowest_f_cost.g;
                let f =  g + h;
                
                if searched_set.0.insert(i) {
                    searched_vec.0.push(AStarCell { 
                        coordinate: i, 
                        f, 
                        g,
                        came_from: Some(lowest_f_cost.coordinate),
                    });

                    image.data[index] = 0;
                    image.data[index + 1] = 255;
                    image.data[index + 2] = 0;
                    image.data[index + 3] = 255;
                    
                } else {
                    for a in &mut searched_vec.0 {
                        if a.coordinate == i {
                            if a.f > f {
                                a.f = f;
                                a.came_from = Some(lowest_f_cost.coordinate);
                            }
                        }
                    }
                }

                if h == 0 {
                    
                    let mut current_coordinate = i;
                    let mut should_break = false;

                    println!("{:?}", searched_vec.0);
                    println!("{:?}", current_coordinate);


                    while !should_break {

                        index = current_coordinate.0 * 4 + (current_coordinate.1 * MAP_SIZE * 8 * 4);

                        image.data[index] = 255;
                        image.data[index + 1] = 0;
                        image.data[index + 2] = 0;
                        image.data[index + 3] = 255;
                        
                        for a in &searched_vec.0 {

                            println!("{:?}", a.coordinate);

                            if a.coordinate.0 as i32 == current_coordinate.0 as i32 && a.coordinate.1 as i32 == current_coordinate.1 as i32 {

                                print!("yes");

                                if let Some(last_coordinate) = a.came_from {
                                    current_coordinate = last_coordinate
                                } else {
                                    should_break = true;
                                }
                            }

                        }
                    }

                    app_state.set(AppState::Finished).unwrap();
                }
            }
        }
    // }
}

fn find_neighbors(pixel: (usize, usize), image_width: i32) -> Vec<(usize, usize)> {

    let mut indexes = Vec::new();
    let pixel_x = pixel.0 as i32;
    let pixel_y = pixel.1 as i32;

    for x in (0..3).map(|x| x - 1) {
        for y in (0..3).map(|y| y - 1) {
            if x == y && x == 0 {
                continue;
            } else {

                let new_x = pixel_x + x;
                let new_y = pixel_y + y;

                if new_x < 0 || pixel_y < 0 || new_x >= image_width || new_y >= image_width {
                    continue;
                } else {

                    indexes.push((new_x as usize, new_y as usize))
                }
            }
        }
    }
    indexes
}

fn distance_between_indexes (
    cell_a: (usize, usize),
    cell_b: (usize, usize),
) -> usize {
    // finds the walkable distance between two indexes in a 1d vector with a width
    let mut x = 0;
    let mut y = 0;

    if cell_a.0 > cell_b.0 {
        x = cell_a.0 - cell_b.0;
    } else {
        x = cell_b.0 - cell_a.0;
    }    
    
    if cell_a.1 > cell_b.1 {
        y = cell_a.1 - cell_b.1;
    } else {
        y = cell_b.1 - cell_a.1;
    }

    if x > y {
        ((x - y) * 10) + y * 14
    } else {
        ((y - x) * 10) + x * 14
    }
}