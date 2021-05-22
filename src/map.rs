use crate::components::{Position, BoxColour};
use crate::entities::*;
use crate::sokoban::ImageAssets;
use amethyst::prelude::World;

pub const TILE_WIDTH: f32 = 32.0;
pub const MAP_WIDTH: u8 = 8;
pub const MAP_HEIGHT: u8 = 9;

pub fn load_map(world: &mut World, map_string: String, assets: &ImageAssets) {
    // rev() because y coordinates are switched!
    let rows: Vec<&str> = map_string
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .rev()
        .collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0,
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
                "BB" => {
                    create_floor(world, position, assets.floor_sprite.clone());
                    create_box(world, position, assets.box_blue_sprite.clone(), BoxColour::Blue);
                }
                "RB" => {
                    create_floor(world, position, assets.floor_sprite.clone());
                    create_box(world, position, assets.box_red_sprite.clone(), BoxColour::Red);
                }
                "BS" => {
                    create_floor(world, position, assets.floor_sprite.clone());
                    create_box_spot(world, position, assets.box_spot_blue_sprite.clone(), BoxColour::Blue);
                }
                "RS" => {
                    create_floor(world, position, assets.floor_sprite.clone());
                    create_box_spot(world, position, assets.box_spot_red_sprite.clone(), BoxColour::Red);
                }
                "N" => (),
                c => panic!("unrecognized map item! {}", c),
            }
        }
    }
}
