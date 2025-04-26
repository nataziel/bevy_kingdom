use crate::{moon::MoonHouse, people::Name, royalty::Royalty, state::RunState};
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
    query: Query<(&Name, Option<&Royalty>), With<Alive>>,
) {
    for event in ev_death.read() {
        if let Ok((name, royalty)) = query.get(event.dying) {
            debug!("Handling death event for {} {}", name.first, name.last);

            commands
                .entity(event.dying)
                .remove::<Alive>()
                .insert(Deceased);

            info!(
                "{} {} died. Cause of death: {}",
                name.first, name.last, event.cause
            );

            if let Some(royalty) = royalty {
                info!(
                    "{} {} is a {:?}! We gotta handle royal death",
                    name.first, name.last, royalty.title
                );
            }
        } else {
            debug!(
                "Can't handle death event for {}, they're probably already dead!",
                event.dying
            )
        }

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
        app.add_systems(
            Update,
            (handle_death, handle_cheat_death).run_if(in_state(RunState::Running)),
        )
        .add_event::<DeathEvent>()
        .add_event::<CheatDeathEvent>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_death() {
        let mut app = App::new();

        app.add_event::<DeathEvent>();

        app.add_systems(Update, handle_death);

        let test_entity = app
            .world_mut()
            .spawn((
                Alive,
                Name {
                    first: "test".into(),
                    last: "guy".into(),
                },
            ))
            .id();

        app.world_mut()
            .resource_mut::<Events<DeathEvent>>()
            .send(DeathEvent::new(test_entity, "Test"));

        app.update();

        assert!(app.world().get::<Alive>(test_entity).is_none());
        assert!(app.world().get::<Deceased>(test_entity).is_some());
    }

    #[test]
    fn test_handle_death_already_dead() {
        let mut app = App::new();

        app.add_event::<DeathEvent>();

        app.add_systems(Update, handle_death);

        let test_entity = app.world_mut().spawn(Deceased).id();

        app.world_mut()
            .resource_mut::<Events<DeathEvent>>()
            .send(DeathEvent::new(test_entity, "Test"));

        app.update();

        assert!(app.world().get::<Alive>(test_entity).is_none());
        assert!(app.world().get::<Deceased>(test_entity).is_some());
    }
}
