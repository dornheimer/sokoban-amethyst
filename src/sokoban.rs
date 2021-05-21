use crate::components::register_components;
use crate::map::load_map;
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst::ui::{TtfFormat, UiTransform, Anchor, UiText, LineMode};
use amethyst::core::ecs::Entity;
use std::fmt;
use std::fmt::Display;

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

        world.insert(Gameplay::default());

        initialise_ui(world);
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32
}

pub enum GameplayState {
    Playing,
    Won
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won"
        })?;
        Ok(())
    }
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
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

pub struct ImageAssets {
    pub player_sprite: SpriteRender,
    pub box_sprite: SpriteRender,
    pub box_spot_sprite: SpriteRender,
    pub wall_sprite: SpriteRender,
    pub floor_sprite: SpriteRender,
}

fn load_assets(world: &mut World) -> ImageAssets {
    ImageAssets {
        player_sprite: create_sprite_render(world, "images/player.png", "images/player.ron"),
        box_sprite: create_sprite_render(world, "images/box.png", "images/box.ron"),
        box_spot_sprite: create_sprite_render(world, "images/box_spot.png", "images/box_spot.ron"),
        wall_sprite: create_sprite_render(world, "images/wall.png", "images/wall.ron"),
        floor_sprite: create_sprite_render(world, "images/floor.png", "images/floor.ron"),
    }
}

fn create_sprite_render(
    world: &mut World,
    asset_path: &str,
    sprite_definition_path: &str,
) -> SpriteRender {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();

    let texture_handle = loader.load(asset_path, ImageFormat::default(), (), &texture_storage);

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

    SpriteRender {
        sprite_sheet: sheet_handle.clone(),
        sprite_number: 0,
    }
}

pub struct GameUi {
    pub moves_element: Entity,
    pub gameplay_state_element: Entity,
}

fn initialise_ui(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let moves_transform = UiTransform::new(
        "MOVES".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        300.,
        50.,
    );

    let gameplay_state_transform = UiTransform::new(
        "MOVES".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.,
        -200.,
        1.,
        300.,
        50.,
    );

    let moves_element = world
        .create_entity()
        .with(moves_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let gameplay_state_element = world
        .create_entity()
        .with(gameplay_state_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(GameUi { moves_element, gameplay_state_element });
}
