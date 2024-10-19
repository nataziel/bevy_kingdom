mod calendar;
mod moon;
mod people;
mod weather;
mod season;

use bevy::prelude::*;
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
        .run();
}
