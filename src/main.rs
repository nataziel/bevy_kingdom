mod calendar;
mod moon;
mod people;
mod season;
mod weather;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use calendar::DatePlugin;
use moon::MoonPlugin;
use people::HelloPlugin;
use season::SeasonPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(HelloPlugin)
        .add_plugins(DatePlugin)
        .add_plugins(MoonPlugin)
        .add_plugins(SeasonPlugin)
        .add_plugins(LogPlugin {
            level: Level::DEBUG,
            filter: "".to_string(),
            custom_layer: |_| None,
        })
        .run();
}
