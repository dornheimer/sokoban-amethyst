use amethyst::ecs::{Component, DenseVecStorage, NullStorage};
use amethyst::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
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

pub fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
    world.register::<Position>();
}
