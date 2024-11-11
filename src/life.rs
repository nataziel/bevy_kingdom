use crate::{moon::MoonHouse, people::Name};
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Alive;

#[derive(Component, Debug)]
struct Deceased;

#[derive(Event, Debug)]
pub struct DeathEvent {
    dying: Entity,
    cause: String,
}

impl DeathEvent {
    pub fn new(dying: Entity, cause: &str) -> Self {
        DeathEvent {
            dying,
            cause: cause.into(),
        }
    }
}

#[derive(Event, Debug)]
pub struct CheatDeathEvent {
    cheater: Entity,
    house: MoonHouse,
}

impl CheatDeathEvent {
    pub fn new(cheater: Entity, house: MoonHouse) -> Self {
        CheatDeathEvent { cheater, house }
    }
}

fn handle_death(
    mut commands: Commands,
    mut ev_death: EventReader<DeathEvent>,
    query: Query<&Name, With<Alive>>,
) {
    for event in ev_death.read() {
        let name = query.get(event.dying).unwrap();
        debug!("Handling death event for {} {}", name.first, name.last);

        commands
            .entity(event.dying)
            .remove::<Alive>()
            .insert(Deceased);

        info!(
            "{} {} died. Cause of death: {}",
            name.first, name.last, event.cause
        )

        // TODO: make the parents/siblings/children sad
    }
}

fn handle_cheat_death(
    mut ev_cheat_death: EventReader<CheatDeathEvent>,
    query: Query<&Name, With<Alive>>,
) {
    for event in ev_cheat_death.read() {
        let name = query.get(event.cheater).unwrap();
        debug!(
            "Handling cheat death event for {} {}",
            name.first, name.last
        );
        // TODO: make the houses get mad at each other?
        // When someone cheats death, house death gets mad at the house that helped them
    }
}

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_death, handle_cheat_death))
            .add_event::<DeathEvent>()
            .add_event::<CheatDeathEvent>();
    }
}
