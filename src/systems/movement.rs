use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReaderId, Write, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode, InputEvent};

use crate::sokoban::{WINDOW_HEIGHT, WINDOW_WIDTH, TILE_WIDTH, Player, Position, Movable, Immovable};
use amethyst::core::shrev::EventChannel;
use amethyst::core::ecs::Entities;
use std::collections::HashMap;
use amethyst::core::ecs::world::Index;
use amethyst::shrev::EventIterator;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[derive(SystemDesc)]
pub struct MovementSystem {
    pub reader_id: Option<ReaderId<InputEvent<StringBindings>>>,
}

impl MovementSystem {

}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Movable>,
        ReadStorage<'s, Immovable>,
        WriteStorage<'s, Position>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>
    );

    fn run(&mut self, (mut transforms, entities, players, movables, immovables, mut positions, mut event_channel): Self::SystemData) {

        let mut to_move = Vec::new();
        let mut mov: HashMap<(u8, u8), Index> = (&entities, &movables, &transforms)
            .join()
            .map(|t| ((t.2.translation().x as u8, t.2.translation().y as u8), t.0.id()))
            .collect::<HashMap<_, _>>();
        let mut immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &transforms)
            .join()
            .map(|t| ((t.2.translation().x as u8, t.2.translation().y as u8), t.0.id()))
            .collect::<HashMap<_, _>>();

        for (_player, transform) in (&players, &mut transforms).join() {
            let event_iterator = event_channel.read(self.reader_id.as_mut().unwrap()).into_iter();
            if let Some(direction) = get_direction(event_iterator) {
                println!("{:?}", direction);
                let (start, end, is_x) = match direction {
                    Direction::Up => (transform.translation().y as u8, WINDOW_HEIGHT as u8, false),
                    Direction::Down => (transform.translation().y as u8, 0, false),
                    Direction::Right => (transform.translation().x as u8, WINDOW_WIDTH as u8, true),
                    Direction::Left => (transform.translation().x as u8, 0, true),
                };

                println!("start {}, end {}, is_x {}", start, end, is_x);

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, transform.translation().y as u8)
                    } else {
                        (transform.translation().x as u8, x_or_y)
                    };

                    match mov.get(&pos) {
                        Some(id) => to_move.push((direction, id.clone())),
                        None => {
                            match immov.get(&pos) {
                                Some(_id) => to_move.clear(),
                                None => break,
                            }
                        }
                    }
                }
            }
        }

        for (direction, id) in to_move {
            let transform = transforms.get_mut(entities.entity(id));
            if let Some(transform) = transform {
                match direction {
                    Direction::Up => transform.prepend_translation_y(TILE_WIDTH),
                    Direction::Down => transform.prepend_translation_y(-TILE_WIDTH),
                    Direction::Right => transform.prepend_translation_x(TILE_WIDTH),
                    Direction::Left => transform.prepend_translation_x(-TILE_WIDTH),
                };
            }
        }

            // for event in channel_bindings.read(self.reader_id.as_mut().unwrap()) {
            //     let direction = get_direction(event);
            //     let (start, end, is_x) = match Direction {
            //         Direction::Up => (transform.y, 0, false),
            //         Direction::Down => (transform.y, HEIGHT, false),
            //         Direction::Right => (transform.x, 0, true),
            //         Direction::Left => (transform.x, WIDTH, true),
            //         _ => continue,
            //     };

                // match event {
                //     InputEvent::KeyPressed {
                //         key_code: VirtualKeyCode::Up,
                //         scancode: 103,
                //     } => { transform.prepend_translation_y(TILE_WIDTH); },
                //     InputEvent::KeyPressed {
                //         key_code: VirtualKeyCode::Down,
                //         scancode: 108,
                //     } => { transform.prepend_translation_y(-TILE_WIDTH); },
                //     InputEvent::KeyPressed {
                //         key_code: VirtualKeyCode::Right,
                //         scancode: 106,
                //     } => { transform.prepend_translation_x(TILE_WIDTH); },
                //     InputEvent::KeyPressed {
                //         key_code: VirtualKeyCode::Left,
                //         scancode: 105,
                //     } => { transform.prepend_translation_x(-TILE_WIDTH); },
                //     _ => ()
                // }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        self.reader_id = Some(world.fetch_mut::<EventChannel<InputEvent<StringBindings>>>().register_reader());
    }
}

fn get_direction(events: EventIterator<InputEvent<StringBindings>>) -> Option<Direction> {
    for event in events {
        match event {
            InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Up,
                scancode: 103,
            } => return Some(Direction::Up),
            InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Down,
                scancode: 108,
            } => return Some(Direction::Down),
            InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Right,
                scancode: 106,
            } => return Some(Direction::Right),
            InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Left,
                scancode: 105,
            } => return Some(Direction::Left),
            _ => return None
        };
    }

    None
}