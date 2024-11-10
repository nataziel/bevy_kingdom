use crate::life::DeathEvent;
use crate::people::{Children, Name, Person, PersonBundle, Siblings};
use bevy::{prelude::*, utils::HashSet};
use rand::{distributions::Bernoulli, prelude::*};
use statrs::distribution::{Continuous, Normal};

#[derive(Component, Debug)]
pub struct ChildBearing;

pub const HUMAN_PREGNANCY_LENGTH: i32 = 266;
pub const HUMAN_PREGNANCY_STD: i32 = 16;

#[derive(Component, Debug)]
pub struct Pregnancy {
    mean_term: i32,
    std_term: i32,
    term: i32,
    progress: i32,
    father: Entity,
}

impl Pregnancy {
    pub fn new(mean_term: i32, std_term: i32, father: Entity) -> Self {
        let mut rng = thread_rng();
        let norm_dist = Normal::new(mean_term.into(), std_term.into()).unwrap();
        let term = norm_dist.sample(&mut rng) as i32;

        Pregnancy {
            mean_term,
            std_term,
            term,
            progress: 0,
            father,
        }
    }
}

#[derive(Event, Debug)]
struct GiveBirthEvent {
    mother: Entity,
    father: Entity,
    progress: i32,
    mean_term: i32,
    std_term: i32,
}

#[derive(Event, Debug)]
struct SuccessfulBirthEvent {
    mother: Entity,
    father: Entity,
}

#[derive(Event, Debug)]
struct UnsuccessfulBirthEvent {
    mother: Entity,
    father: Entity,
    term_diff: i32,
}

fn handle_pregnancy(
    mut ev_give_birth: EventWriter<GiveBirthEvent>,
    mut query: Query<(Entity, &Name, &mut Pregnancy), (With<Person>, With<ChildBearing>)>,
) {
    for (mother, name, mut pregnancy) in &mut query {
        pregnancy.progress += 1;
        debug!(
            "{} {} is pregnant, {}/{}",
            name.first, name.last, pregnancy.progress, pregnancy.mean_term
        );

        if pregnancy.progress >= pregnancy.term {
            ev_give_birth.send(GiveBirthEvent {
                mother,
                father: pregnancy.father,
                progress: pregnancy.progress,
                mean_term: pregnancy.mean_term,
                std_term: pregnancy.std_term,
            });
        }
    }
}

fn handle_give_birth(
    mut commands: Commands,
    mut ev_give_birth: EventReader<GiveBirthEvent>,
    mut ev_successful_birth: EventWriter<SuccessfulBirthEvent>,
    mut ev_unsuccessful_birth: EventWriter<UnsuccessfulBirthEvent>,
) {
    for event in ev_give_birth.read() {
        debug!("Handling give_birth for {}", event.mother);
        commands.entity(event.mother).remove::<Pregnancy>();

        let term_diff = event.progress - event.mean_term;
        debug!("Term_Diff: {}", term_diff);
        let problem_dist =
            // double the standard deviation just to make it less likely there are problems
            Normal::new(event.mean_term.into(), (2 * event.std_term).into()).unwrap();
        let pdf_at_sample = problem_dist.pdf(event.progress.into());
        debug!("pdf_at_sample: {}", pdf_at_sample);
        let pdf_at_mean = problem_dist.pdf(event.mean_term.into());
        debug!("pdf_at_mean: {}", pdf_at_mean);

        let mut rng = thread_rng();
        let bernoulli_dist = Bernoulli::new(pdf_at_sample / pdf_at_mean).unwrap();
        let successful_birth = bernoulli_dist.sample(&mut rng);
        debug!("Outcome of bernoulli trial {}", successful_birth);

        match successful_birth {
            true => {
                ev_successful_birth.send(SuccessfulBirthEvent {
                    mother: event.mother,
                    father: event.father,
                });
            }
            false => {
                ev_unsuccessful_birth.send(UnsuccessfulBirthEvent {
                    mother: event.mother,
                    father: event.father,
                    term_diff,
                });
            }
        }
    }
}

fn handle_successful_birth(
    mut commands: Commands,
    mut ev_successful_birth: EventReader<SuccessfulBirthEvent>,
    mut query_parents: Query<(&mut Children, &Name)>,
    mut query_siblings: Query<&mut Siblings>,
) {
    for event in ev_successful_birth.read() {
        // create a set of parents for the new child
        let new_child_parents = HashSet::from([event.mother, event.father]);

        // create a set of siblings for the new child
        let mut new_child_siblings = HashSet::new();
        // get the children of the mother
        if let Ok((children_mother, _)) = query_parents.get(event.mother) {
            for child in &children_mother.set {
                new_child_siblings.insert(*child);
            }
        }
        // get the children of the father
        if let Ok((children_father, _)) = query_parents.get(event.father) {
            for child in &children_father.set {
                new_child_siblings.insert(*child);
            }
        }

        // TODO: generalise this
        let new_child = commands
            .spawn(PersonBundle::new_child(
                "Penny",
                "Morales-Allan",
                new_child_parents,
                // gotta clone cos we're gonna use it again later
                new_child_siblings.clone(),
            ))
            .id();

        // add the kid to the hashset of children for each parent
        if let Ok((mut children_mother, name_mother)) = query_parents.get_mut(event.mother) {
            info!("{} {} gave birth!", name_mother.first, name_mother.last);
            children_mother.set.insert(new_child);
        }
        if let Ok((mut children_father, _name_father)) = query_parents.get_mut(event.father) {
            children_father.set.insert(new_child);
        }
        // insert the new child into the set of siblings for each of their siblings
        let mut siblings_iter = query_siblings.iter_many_mut(&new_child_siblings);
        while let Some(mut siblings_of_sibling) = siblings_iter.fetch_next() {
            siblings_of_sibling.set.insert(new_child);
        }
    }
}

fn handle_unsuccessful_birth(
    mut ev_unsuccessful_birth: EventReader<UnsuccessfulBirthEvent>,
    mut ev_death: EventWriter<DeathEvent>,
) {
    // TODO: make some fucked up shit happen
    // mum dies? baby dies? :(
    for event in ev_unsuccessful_birth.read() {
        debug!("Handling unsuccessful birth {:?}", event);
        debug!("Term Diff: {}", event.term_diff);
        if event.term_diff >= 0 {
            ev_death.send(DeathEvent::new(event.mother));
            // make the dad get sad?
        } else {
            debug!("Term dif too low");
            // make both mum and dad bet sad
        }
    }
}

pub struct ReproductionPlugin;

impl Plugin for ReproductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (handle_pregnancy, handle_give_birth).chain(),
                (handle_successful_birth, handle_unsuccessful_birth),
            )
                .chain(),
        )
        .add_event::<GiveBirthEvent>()
        .add_event::<SuccessfulBirthEvent>()
        .add_event::<UnsuccessfulBirthEvent>();
    }
}
