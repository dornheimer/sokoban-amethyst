use crate::components::*;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::SpriteRender;

use crate::map::TILE_WIDTH;
use crate::sokoban::MyPrefabData;
use amethyst::assets::{Handle, Prefab};

pub fn create_wall(world: &mut World, position: Position, sprite: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        position.x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        position.y as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        10.0,
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

pub fn create_box(world: &mut World, position: Position, sprite: SpriteRender, colour: BoxColour, box_prefab: &Handle<Prefab<MyPrefabData>>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        position.x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        position.y as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        10.0,
    );

    world
        .create_entity()
        .with(transform)
        .with(box_prefab.clone())
        .with(sprite.clone())
        .with(Box { colour })
        .with(Movable)
        .with(position)
        .build();
}

pub fn create_box_spot(
    world: &mut World,
    position: Position,
    sprite: SpriteRender,
    colour: BoxColour,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        position.x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        position.y as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        9.0,
    );

    world
        .create_entity()
        .with(transform)
        .with(sprite.clone())
        .with(BoxSpot { colour })
        .with(position)
        .build();
}

pub fn create_floor(world: &mut World, position: Position, sprite: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        position.x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        position.y as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        5.0,
    );

    world
        .create_entity()
        .with(transform)
        .with(sprite.clone())
        .with(position)
        .build();
}

pub fn create_player(
    world: &mut World,
    position: Position,
    sprite: SpriteRender,
    player_prefab: &Handle<Prefab<MyPrefabData>>,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        position.x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        position.y as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH,
        10.0,
    );

    world
        .create_entity()
        .with(player_prefab.clone())
        .with(transform)
        .with(sprite.clone())
        .with(Player {})
        .with(Movable)
        .with(position)
        .build();
}
