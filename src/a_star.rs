use bevy::{prelude::*, utils::HashSet};

use crate::{AppState, setup::StartEnd, MAP_SIZE};

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
}

#[derive(Resource)]
pub struct SearchedCellsVec (Vec<AStarCell>);

#[derive(Resource)]
pub struct SearchedCellsSet (HashSet<AStarCell>);

fn setup (
    mut commands: Commands,
    start_end: Res<StartEnd>,
) {
    let mut set = HashSet::new();
    let mut vec = Vec::new();

    let start = start_end.start.unwrap();
    let end = start_end.end.unwrap();
    let h = distance_between_indexes(start, end);
    let cell = AStarCell { coordinate: start, f: h };

    vec.push(cell.clone());
    set.insert(cell);

    commands.insert_resource(SearchedCellsVec(vec));
    commands.insert_resource(SearchedCellsSet(set));
}

// each node has
// G cost -> distance from start
// H cost -> distance from end
// F cost -> G + H

fn a_star (
    start_end: Res<StartEnd>,
    mut searched_vec: ResMut<SearchedCellsVec>,
    mut searched_set: ResMut<SearchedCellsSet>,
) {

    searched_vec.0.sort_by_key(|a| a.f);

    let new_cells = find_neighbors(searched_vec[0], MAP_SIZE * 8);

    for i in new_cells {
        if searched_set.0.len() != searched_set.0.insert(i).len() {
        }
    }
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
    let mut distance = 0;

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
        distance = ((x - y) * 10) + y * 14
    } else {
        distance = ((y - x) * 10) + x * 14
    }

    distance

}