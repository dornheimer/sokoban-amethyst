use amethyst::animation::AnimationBundle;
use amethyst::assets::PrefabLoaderSystemDesc;
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::renderer::SpriteRender;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::sokoban::{AnimationId, MyPrefabData, Sokoban};
use amethyst::audio::AudioBundle;
use amethyst::utils::fps_counter::FpsCounterBundle;

mod components;
mod entities;
mod map;
mod sokoban;
mod systems;
mod events;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<MyPrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_bundle(FpsCounterBundle)?
        .with(
            systems::MovementSystem { input_reader: None },
            "movement_system",
            &["input_system"],
        )
        .with(
            systems::GameplayStateSystem {},
            "gameplay_state_system",
            &[],
        )
        .with(systems::AnimationSystem {}, "animation_system", &[])
        .with(systems::SoundSystem { move_reader: None }, "sound_system", &[]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Sokoban, game_data)?;
    game.run();

    Ok(())
}
