use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Alive;

#[derive(Component, Debug)]
struct Deceased;

#[derive(Event, Debug)]
pub struct DeathEvent {
    dying: Entity,
}

impl DeathEvent {
    pub fn new(dying: Entity) -> Self {
        DeathEvent { dying }
    }
}

fn handle_death(mut commands: Commands, mut ev_death: EventReader<DeathEvent>) {
    for event in ev_death.read() {
        debug!("Handling death event for {:?}", event.dying);

        commands
            .entity(event.dying)
            .remove::<Alive>()
            .insert(Deceased);

        // TODO: make the parents/siblings/children sad
    }
}

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_death)
            .add_event::<DeathEvent>();
    }
}
