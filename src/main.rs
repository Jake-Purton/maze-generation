mod depth_first;

use bevy::{prelude::*, render::{texture::ImageSampler, render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}}};
use depth_first::{Map, depth_first_search, data_from_map};
use rand::Rng;

const MAP_SIZE: usize = 16;
const SCREEN_SIZE: Vec2 = Vec2 { x: 800.0, y: 800.0 };

#[derive(Resource)]
pub struct MazeMap(Vec<u8>);

fn main() {

    let mut map: Map = Map::new(MAP_SIZE);
    let x = rand::thread_rng().gen_range(0..MAP_SIZE);
    let y = rand::thread_rng().gen_range(0..MAP_SIZE);
    depth_first_search(&mut map, x, y, None);

    let image_data = data_from_map(map);

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: SCREEN_SIZE.x,
                        height: SCREEN_SIZE.y,
                        title: "To do".to_string(),
                        resizable: false,
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSampler::nearest_descriptor(),
                }),
        )
        .insert_resource(MazeMap(image_data))
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    data: Res<MazeMap>,
    mut images: ResMut<Assets<Image>>,
) {
    commands
        .spawn(Camera2dBundle::default());

    let mut image = Image::new(
        Extent3d {
            width: (MAP_SIZE * 8) as u32,
            height: (MAP_SIZE * 8) as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2, 
        data.0.clone(), 
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let image = images.add(image.clone());
    
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(SCREEN_SIZE),
            ..default()
        },
        texture: image,
        ..default()
    });
}