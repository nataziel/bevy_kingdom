use bevy::prelude::*;

use crate::{
    age::Age,
    life::Alive,
    people::{Name, Person},
};

#[derive(Debug)]
pub enum Title {
    King,
    Queen,
    Prince,
    Princess,
    Duke,
    Duchess,
}

#[derive(Component, Debug)]
pub struct Royalty {
    pub title: Title,
}

fn detail_royalty(query: Query<(&Name, &Age, &Royalty), (With<Person>, With<Alive>)>) {
    debug!("~~ Current Royalty ~~");
    for (name, age, royalty) in &query {
        info!(
            "{:?}: {} {} ({})",
            royalty.title, name.first, name.last, age
        );
    }
}

pub struct RoyaltyPlugin;

impl Plugin for RoyaltyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detail_royalty);
    }
}
