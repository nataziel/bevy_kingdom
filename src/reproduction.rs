use crate::people::{Children, Name, Parents, Person};
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ChildBearing;

pub const HUMAN_PREGNANCY_LENGTH: i32 = 266;

#[derive(Component, Debug)]
pub struct Pregnancy {
    mean_term: i32,
    progress: i32,
    father: Entity,
}

impl Pregnancy {
    pub fn new(mean_term: i32, father: Entity) -> Self {
        Pregnancy {
            mean_term,
            progress: 0,
            father,
        }
    }
}

#[derive(Event, Debug)]
pub struct GiveBirthEvent {
    mother: Entity,
    father: Entity,
}

fn handle_pregnancy(
    mut ev_give_birth: EventWriter<GiveBirthEvent>,
    mut query: Query<(Entity, &Name, &mut Pregnancy), (With<Person>, With<ChildBearing>)>,
) {
    for (mother, name, mut pregnancy) in &mut query {
        pregnancy.progress += 1;
        info!(
            "{} {} is pregnant, {}/{}",
            name.first, name.last, pregnancy.progress, pregnancy.mean_term
        );
        if pregnancy.progress >= pregnancy.mean_term {
            ev_give_birth.send(GiveBirthEvent {
                mother,
                father: pregnancy.father,
            });
        }
    }
}

fn handle_givebirth(
    mut commands: Commands,
    mut ev_give_birth: EventReader<GiveBirthEvent>,
    mut query_parents: Query<&mut Children>,
) {
    // TODO: generalise this
    for event in ev_give_birth.read() {
        debug!("Entity {:?} gave birth!", event.mother);
        commands.entity(event.mother).remove::<Pregnancy>();
        let penny = commands
            .spawn((
                Person,
                Name {
                    first: "Penny".to_string(),
                    last: "Morales-Allan".to_string(),
                },
                Parents {
                    list: vec![event.mother, event.father],
                },
            ))
            .id();

        if let Ok(mut children_mother) = query_parents.get_mut(event.mother) {
            children_mother.list.push(penny)
        }
        if let Ok(mut children_father) = query_parents.get_mut(event.father) {
            children_father.list.push(penny)
        }
    }
}

pub struct ReproductionPlugin;

impl Plugin for ReproductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_pregnancy, handle_givebirth).chain());
    }
}
