mod depth_first;

use bevy::prelude::*;
use depth_first::{Map, depth_first_search};
use rand::Rng;

const MAP_SIZE: usize = 8;

#[derive(Resource)]
pub struct MazeMap(Map);

fn main() {

    let mut map: Map = Map::new(MAP_SIZE);
    let x = rand::thread_rng().gen_range(0..MAP_SIZE);
    let y = rand::thread_rng().gen_range(0..MAP_SIZE);
    depth_first_search(&mut map, x, y, None);

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
        .insert_resource(MazeMap(map))
        .run();
}