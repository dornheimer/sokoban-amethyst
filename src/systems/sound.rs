use amethyst::core::ecs::{System, ReaderId, World, Read, Entities, ReadStorage, Write, Join, ReadExpect};
use crate::events::{MoveEvent, EntityMoved, BoxPlacedOnSpot};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::SystemData;
use crate::components::*;
use std::collections::HashMap;
use amethyst::audio::{Source, SourceHandle};
use amethyst::assets::{AssetStorage};
use amethyst::audio::output::Output;
use crate::sokoban::Sounds;

pub struct SoundSystem {
    pub(crate) move_reader: Option<ReaderId<MoveEvent>>
}

impl<'s> System<'s> for SoundSystem {
    type SystemData = (
        Write<'s, EventChannel<MoveEvent>>,
        Entities<'s>,
        ReadStorage<'s, Box>,
        ReadStorage<'s, BoxSpot>,
        ReadStorage<'s, Position>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut move_events, entities, boxes, box_spots, positions, storage, sounds, output) = data;

        let mut new_events = Vec::new();
        {
            for event in move_events.read(self.move_reader.as_mut().unwrap()) {
                println!("New event: {:?}", event);

                match event {
                    MoveEvent::PlayerHitObstacle => {
                        play_sound(&sounds.wall, &storage, output.as_deref());
                    }
                    MoveEvent::EntityMoved(EntityMoved { id }) => {
                        if let Some(the_box) = boxes.get(entities.entity(*id)) {
                            let box_spots_with_positions: HashMap<(u8, u8), &BoxSpot> =
                                (&box_spots, &positions)
                                    .join()
                                    .map(|t| ((t.1.x, t.1.y), t.0))
                                    .collect::<HashMap<_, _>>();

                            if let Some(box_position) = positions.get(entities.entity(*id)) {
                                if let Some(box_spot) =
                                box_spots_with_positions.get(&(box_position.x, box_position.y))
                                {
                                    new_events.push(MoveEvent::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                        is_correct_spot: (box_spot.colour == the_box.colour)
                                    }));
                                }
                            }
                        }
                    }
                    MoveEvent::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                        if *is_correct_spot {
                            play_sound(&sounds.correct, &storage, output.as_deref());
                        } else {
                            play_sound(&sounds.incorrect, &storage, output.as_deref());
                        }
                    }
                }
            }

            move_events.iter_write(new_events);
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        self.move_reader = Some(
            world
                .fetch_mut::<EventChannel<MoveEvent>>()
                .register_reader(),
        );
    }
}

pub fn play_sound(sound: &SourceHandle, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(sound) {
            output.play_once(sound, 1.0);
        }
    }
}