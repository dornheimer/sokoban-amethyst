use amethyst::core::ecs::{System, Write, ReadStorage, Join, WriteStorage, ReadExpect};
use crate::sokoban::{Gameplay, GameplayState, GameUi};
use crate::components::{Position, BoxSpot, Box};
use std::collections::HashMap;
use amethyst::ui::UiText;

pub struct GameplayStateSystem {}

impl<'s> System<'s> for GameplayStateSystem {
    type SystemData = (
        Write<'s, Gameplay>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Box>,
        ReadStorage<'s, BoxSpot>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, GameUi>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay_state, positions, boxes, box_spots, mut ui_text, game_ui) = data;

        // update ui elements
        if let Some(moves_count_ui) = ui_text.get_mut(game_ui.moves_element) {
            moves_count_ui.text = format!("Moves: {}", gameplay_state.moves_count.to_string());
        }

        if let Some(gameplay_state_ui) = ui_text.get_mut(game_ui.gameplay_state_element) {
            gameplay_state_ui.text = format!("{}", gameplay_state.state.to_string());
        }

        let boxes_by_position: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_,_>>();

        for (_box_spot, position) in (&box_spots, &positions).join() {
            if !boxes_by_position.contains_key(&(position.x, position.y)) {
                gameplay_state.state = GameplayState::Playing;
                return;
            }
        }

        gameplay_state.state = GameplayState::Won;
    }
}