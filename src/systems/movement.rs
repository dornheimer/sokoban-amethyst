use std::collections::HashMap;

use amethyst::core::ecs::world::Index;
use amethyst::core::ecs::Entities;
use amethyst::core::shrev::EventChannel;
use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Join, Read, ReadStorage, ReaderId, System, SystemData, World, Write, WriteStorage,
};
use amethyst::input::{InputEvent, StringBindings, VirtualKeyCode};
use amethyst::shrev::EventIterator;

use crate::map::{MAP_HEIGHT, MAP_WIDTH, TILE_WIDTH};
use crate::components::*;
use crate::sokoban::Gameplay;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(SystemDesc)]
pub struct MovementSystem {
    pub reader_id: Option<ReaderId<InputEvent<StringBindings>>>,
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Movable>,
        ReadStorage<'s, Immovable>,
        WriteStorage<'s, Position>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
        Write<'s, Gameplay>,
    );

    fn run(
        &mut self,
        (mut transforms, entities, players, movables, immovables, mut positions, event_channel, mut gameplay): Self::SystemData,
    ) {
        let mut to_move = Vec::new();
        let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
            .join()
            .map(|t| ((t.2.x, t.2.y), t.0.id()))
            .collect::<HashMap<_, _>>();
        let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
            .join()
            .map(|t| ((t.2.x, t.2.y), t.0.id()))
            .collect::<HashMap<_, _>>();

        for (_player, position) in (&players, &mut positions).join() {
            let event_iterator = event_channel
                .read(self.reader_id.as_mut().unwrap())
                .into_iter();
            if let Some(direction) = get_direction(event_iterator) {
                let (start, end, is_x) = match direction {
                    Direction::Up => (position.y, MAP_HEIGHT, false),
                    Direction::Down => (position.y, 0, false),
                    Direction::Right => (position.x, MAP_WIDTH, true),
                    Direction::Left => (position.x, 0, true),
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    match mov.get(&pos) {
                        Some(id) => to_move.push((direction, id.clone())),
                        None => match immov.get(&pos) {
                            Some(_id) => to_move.clear(),
                            None => break,
                        },
                    }
                }
            }
        }

        if to_move.len() > 0 {
            gameplay.moves_count += 1;
        }

        for (direction, id) in to_move {
            let transform = transforms.get_mut(entities.entity(id));
            let position = positions.get_mut(entities.entity(id));
            if let Some(transform) = transform {
                match direction {
                    Direction::Up => {
                        transform.prepend_translation_y(TILE_WIDTH);
                        position.unwrap().y += 1;
                    }
                    Direction::Down => {
                        transform.prepend_translation_y(-TILE_WIDTH);
                        position.unwrap().y -= 1;
                    }
                    Direction::Right => {
                        transform.prepend_translation_x(TILE_WIDTH);
                        position.unwrap().x += 1;
                    }
                    Direction::Left => {
                        transform.prepend_translation_x(-TILE_WIDTH);
                        position.unwrap().x -= 1;
                    }
                };
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        self.reader_id = Some(
            world
                .fetch_mut::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        );
    }
}

fn get_direction(events: EventIterator<InputEvent<StringBindings>>) -> Option<Direction> {
    for event in events {
        return match event {
            InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Up,
                scancode: 103,
            } => Some(Direction::Up),
            InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Down,
                scancode: 108,
            } => Some(Direction::Down),
            InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Right,
                scancode: 106,
            } => Some(Direction::Right),
            InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Left,
                scancode: 105,
            } => Some(Direction::Left),
            _ => None,
        };
    }

    None
}
