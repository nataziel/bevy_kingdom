use crate::life::Alive;
use crate::life::DeathEvent;
use crate::{people::Name, royalty::Royalty, state::RunState};
use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_health.run_if(in_state(RunState::Running)));
    }
}

fn handle_health(
    query: Query<(Entity, &Royalty, &Name), With<Alive>>,
    mut ev_death: EventWriter<DeathEvent>,
) {
    // make a royal die to test handling royal death
    // todo: remove the royal stuff and add health handling
    for (entity, royal, name) in query.iter() {
        info!(
            "Sending death event for {} {}, the {:?}",
            name.first, name.last, royal.title
        );
        ev_death.write(DeathEvent::new(entity, "Royal"));
    }
}
