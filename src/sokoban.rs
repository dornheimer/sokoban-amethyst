use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst::core::math::Vector3;
use amethyst::core::ecs::NullStorage;

pub const WINDOW_HEIGHT: f32 = 600.0;
pub const WINDOW_WIDTH: f32 = 800.0;

pub struct Sokoban;

impl SimpleState for Sokoban {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        register_components(world);
        initialise_camera(world);

        let assets = load_assets(world);
        initialise_level(world, &assets);
    }
}

fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
    world.register::<Position>();
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WINDOW_WIDTH * 0.5, WINDOW_HEIGHT * 0.5, 20.0);

    world
        .create_entity()
        .with(Camera::standard_2d(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with(transform)
        .build();
}

struct ImageAssets {
    player_sprite: SpriteRender,
    box_sprite: SpriteRender,
    box_spot_sprite: SpriteRender,
    wall_sprite: SpriteRender,
    floor_sprite: SpriteRender,
}

fn load_assets(world: &mut World) -> ImageAssets {
    ImageAssets {
        player_sprite: create_sprite_render(world, "images/player.png", "images/player.ron"),
        box_sprite: create_sprite_render(world, "images/box.png", "images/box.ron"),
        box_spot_sprite: create_sprite_render(world, "images/box_spot.png", "images/box_spot.ron"),
        wall_sprite: create_sprite_render(world, "images/wall.png", "images/wall.ron"),
        floor_sprite: create_sprite_render(world, "images/floor.png", "images/floor.ron")
    }
}

pub const TILE_WIDTH: f32 = 32.0;
pub const MAP_WIDTH: u8 = 8;
pub const MAP_HEIGHT: u8 = 9;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Movable;

impl Component for Movable {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Immovable;

impl Component for Immovable {
    type Storage = NullStorage<Self>;
}

pub struct Wall {}

impl Component for Wall {
    type Storage = DenseVecStorage<Self>;
}

pub struct Player {}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub struct Box {}

impl Component for Box {
    type Storage = DenseVecStorage<Self>;
}

pub struct BoxSpot {}

impl Component for BoxSpot {
    type Storage = DenseVecStorage<Self>;
}

fn create_wall(world: &mut World, position: Position, sprite: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        position.x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        position.y as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        10.0
    );

    world
        .create_entity()
        .with(transform)
        .with(sprite.clone())
        .with(Wall {})
        .with(Immovable)
        .with(position)
        .build();
}

fn create_box(world: &mut World, position: Position, sprite: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        position.x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        position.y as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        10.0
    );

    world
        .create_entity()
        .with(transform)
        .with(sprite.clone())
        .with(Box {})
        .with(Movable)
        .with(position)
        .build();
}

fn create_box_spot(world: &mut World, position: Position, sprite: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        position.x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        position.y as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        9.0
    );

    world
        .create_entity()
        .with(transform)
        .with(sprite.clone())
        .with(BoxSpot {})
        .with(position)
        .build();
}

fn create_floor(world: &mut World, position: Position, sprite: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        position.x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        position.y as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        5.0
    );

    world
        .create_entity()
        .with(transform)
        .with(sprite.clone())
        .with(position)
        .build();
}

fn create_player(world: &mut World, position: Position, sprite: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        position.x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        position.y as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        10.0
    );

    world
        .create_entity()
        .with(transform)
        .with(sprite.clone())
        .with(Player {})
        .with(Movable)
        .with(position)
        .build();
}

fn initialise_level(world: &mut World, assets: &ImageAssets) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string(), assets);
}

fn create_sprite_render(world: &mut World, asset_path: &str, sprite_definition_path: &str) -> SpriteRender {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();

    let texture_handle = loader.load(
        asset_path,
        ImageFormat::default(),
        (),
        &texture_storage
    );

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            sprite_definition_path,
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    SpriteRender { sprite_sheet: sheet_handle.clone(), sprite_number: 0 }
}

fn load_map(world: &mut World, map_string: String, assets: &ImageAssets) {
    // rev() because y coordinates are switched!
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).rev().collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0
            };

            match *column {
                "." => create_floor(world, position, assets.floor_sprite.clone()),
                "W" => {
                    create_floor(world, position, assets.floor_sprite.clone());
                    create_wall(world, position, assets.wall_sprite.clone());
                }
                "P" => {
                    create_floor(world, position, assets.floor_sprite.clone());
                    create_player(world, position, assets.player_sprite.clone());
                }
                "B" => {
                    create_floor(world, position, assets.floor_sprite.clone());
                    create_box(world, position, assets.box_sprite.clone());
                }
                "S" => {
                    create_floor(world, position, assets.floor_sprite.clone());
                    create_box_spot(world, position, assets.box_spot_sprite.clone());
                }
                "N" => (),
                c => panic!("unrecognized map item! {}", c)
            }
        }
    }
}

pub fn coordinate_to_trans(coordinate: f32) -> f32 {
    coordinate * TILE_WIDTH + 0.5 * TILE_WIDTH
}