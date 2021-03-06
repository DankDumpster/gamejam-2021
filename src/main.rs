#![allow(clippy::too_many_arguments)]

mod level;
mod models;
mod setup;
mod systems;
mod ui;

use crate::models::player;
use crate::setup::setup;
use crate::systems::{death, gravity, movement, plates};

use crate::systems::movement::MovementTimer;
use bevy::prelude::*;
use bevy_diagnostic::FrameTimeDiagnosticsPlugin;
use log::LevelFilter;
use simple_logger::SimpleLogger;

// TODO Make jetpack toggle

fn main() {
    #[cfg(debug_assertions)]
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    #[cfg(not(debug_assertions))]
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    App::build()
        .insert_resource(WindowDescriptor {
            title: "Rusty Rocket".to_string(),
            vsync: true,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage("level_setup", SystemStage::single(level::spawn.system()))
        .add_startup_stage("player_setup", SystemStage::single(player::spawn.system()))
        .add_system(movement::init.system())
        .add_system(gravity::init.system())
        .add_system(plates::init.system())
        .add_system(death::init.system())
        .add_system(ui::update_text_diagnostic.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(PrintDiagnosticsPlugin::default())
        .run();
}
