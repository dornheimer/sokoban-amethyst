use amethyst::core::ecs::{System, ReadStorage, WriteStorage, Entities, Join};
use amethyst::animation::{get_animation_set, AnimationSet, AnimationControlSet, AnimationCommand, EndControl};
use crate::sokoban::AnimationId;
use amethyst::renderer::SpriteRender;

pub struct AnimationSystem {}

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, animation_sets, mut control_sets) = data;

        for (entity, animation_set) in (&entities, &animation_sets).join() {
            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
            control_set.add_animation(
                AnimationId::Idle,
                &animation_set.get(&AnimationId::Idle).unwrap(),
                EndControl::Loop(None),
                1.0,
                AnimationCommand::Start,
            );
        }
    }
}