#![warn(clippy::all, clippy::pedantic)]
mod age;
mod calendar;
mod frames;
mod life;
mod moon;
mod people;
mod reproduction;
mod season;
mod state;
mod weather;

use age::AgePlugin;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    state::app::StatesPlugin,
};
use calendar::DatePlugin;
use frames::FramePlugin;
use life::LifePlugin;
use moon::MoonPlugin;
use people::HelloPlugin;
use reproduction::ReproductionPlugin;
use season::SeasonPlugin;
use state::PausePlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(StatesPlugin) // needed to use states
        .add_plugins(FramePlugin) // add framecount resource
        .add_plugins(HelloPlugin)
        .add_plugins(AgePlugin)
        .add_plugins(ReproductionPlugin)
        .add_plugins(LifePlugin)
        .add_plugins(DatePlugin)
        .add_plugins(MoonPlugin)
        .add_plugins(SeasonPlugin)
        .add_plugins(PausePlugin) // adds RunState and toggle based on frame count
        .add_plugins(LogPlugin {
            level: Level::DEBUG,
            filter: String::new(),
            custom_layer: |_| None,
        })
        .run();
}
