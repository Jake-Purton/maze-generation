mod a_star;
mod depth_first;
mod setup;

use a_star::AStarPlugin;
use bevy::{asset::HandleId, prelude::*, render::texture::ImageSampler, window::PresentMode};
use depth_first::{data_from_map, depth_first_search, Map};
use rand::Rng;
use setup::{SetupPlugin, StartEnd};

const MAP_SIZE: usize = 8;
const PIXELS_PER_CELL: usize = 16;
const SCREEN_SIZE: Vec2 = Vec2 { x: 800.0, y: 800.0 };

#[derive(Resource)]
pub struct MazeMap(Map);

#[derive(Resource)]
pub struct MazeMapData(Vec<u8>);

#[derive(Resource)]
pub struct MazeMapId(HandleId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum AppState {
    Setup,
    AStar,
    Finished,
}

fn main() {
    let mut map: Map = Map::new(MAP_SIZE);
    let x = rand::thread_rng().gen_range(0..MAP_SIZE);
    let y = rand::thread_rng().gen_range(0..MAP_SIZE);
    depth_first_search(&mut map, x, y, None);

    let image_data = data_from_map(&map);

    App::new()
        .add_state(AppState::Setup)
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: SCREEN_SIZE.x,
                        height: SCREEN_SIZE.y,
                        title: "To do".to_string(),
                        resizable: false,
                        present_mode: PresentMode::AutoNoVsync,
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSampler::nearest_descriptor(),
                }),
        )
        .add_plugin(SetupPlugin)
        .add_plugin(AStarPlugin)
        .insert_resource(MazeMapData(image_data))
        .insert_resource(MazeMap(map))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(StartEnd {
            start: None,
            end: None,
        })
        .run();
}
