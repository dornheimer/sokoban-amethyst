use crate::components::*;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::SpriteRender;

use crate::map::TILE_WIDTH;

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

pub fn create_box(world: &mut World, position: Position, sprite: SpriteRender) {
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
        .with(Box {})
        .with(Movable)
        .with(position)
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position, sprite: SpriteRender) {
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
        .with(BoxSpot {})
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

pub fn create_player(world: &mut World, position: Position, sprite: SpriteRender) {
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
        .with(Player {})
        .with(Movable)
        .with(position)
        .build();
}
