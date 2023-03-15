mod depth_first;

use bevy::prelude::*;
use depth_first::{Map, depth_first_search};
use rand::Rng;

const MAP_SIZE: usize = 16;
const MAP_SCALE: f32 = 16.0;

#[derive(Resource)]
pub struct MazeMap(Map);

fn main() {

    let mut map: Map = Map::new(MAP_SIZE);
    let x = rand::thread_rng().gen_range(0..MAP_SIZE);
    let y = rand::thread_rng().gen_range(0..MAP_SIZE);
    depth_first_search(&mut map, x, y, None);

    startup_draw_map_system(map);

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 800.0,
                title: "To do".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .run();
}

fn startup_draw_map_system (
    map: Map,
) {    

    let mut image: Vec<Vec<[u8; 4]>> = Vec::new();

    for (_, cells) in map.map.iter().enumerate() {

        for i in 0..8 {       
            let mut row: Vec<[u8; 4]> = Vec::new();
            for cell in cells {
                for a in 0..8 {
                    if i == 0 && cell.top {
                        row.push([255, 255, 255, 255]);
                    } else if i == 7 && cell.bottom {
                        row.push([255, 255, 255, 255]);
                    } else if a == 0 && cell.left {
                        row.push([255, 255, 255, 255]);
                    } else if a == 7 && cell.right {
                        row.push([255, 255, 255, 255]);
                    } else {
                        row.push([255, 255, 255, 0]);
                    }
                }
            }
            image.push(row);
        }
    }


    for i in image {
        let mut string = String::new();
        for a in i {
            if a[3] == 255 {
                string.push('#');
            } else {
                string.push(' ');
            }
        }
        println!("{string}")
    }
} 

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default());
}