use crate::people::{Children, Name, Parents, Person, Siblings};
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
    progress: i32,
    father: Entity,
}

impl Pregnancy {
    pub fn new(mean_term: i32, std_term: i32, father: Entity) -> Self {
        Pregnancy {
            mean_term,
            std_term,
            progress: 0,
            father,
        }
    }
}

#[derive(Event, Debug)]
pub struct GiveBirthEvent {
    mother: Entity,
    father: Entity,
    progress: i32,
    mean_term: i32,
    std_term: i32,
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

        let mut rng = thread_rng();
        let norm_dist = Normal::new(pregnancy.mean_term.into(), pregnancy.std_term.into()).unwrap();
        let sampled_value = norm_dist.sample(&mut rng) as i32;

        if pregnancy.progress >= sampled_value {
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

fn handle_givebirth(
    mut commands: Commands,
    mut ev_give_birth: EventReader<GiveBirthEvent>,
    mut query_parents: Query<(&mut Children, &Name)>,
    mut query_siblings: Query<&mut Siblings>,
) {
    for event in ev_give_birth.read() {
        commands.entity(event.mother).remove::<Pregnancy>();

        // create a vector of siblings for the new child
        let mut new_child_siblings = HashSet::new();
        // get the children of the mother
        if let Ok((children_mother, _)) = query_parents.get(event.mother) {
            for child in &children_mother.list {
                new_child_siblings.insert(*child);
            }
        }
        // get the children of the father
        if let Ok((children_father, _)) = query_parents.get(event.father) {
            for child in &children_father.list {
                new_child_siblings.insert(*child);
            }
        }

        // TODO: generalise this
        let new_child = commands
            .spawn((
                Person,
                Name {
                    first: "Penny".to_string(),
                    last: "Morales-Allan".to_string(),
                },
                Parents {
                    list: HashSet::from([event.mother, event.father]),
                },
                Siblings {
                    // gotta clone it cos we are gonna use it again after
                    list: new_child_siblings.clone(),
                },
            ))
            .id();

        // insert the new child into the set of siblings for each of their siblings
        let mut siblings_iter = query_siblings.iter_many_mut(&new_child_siblings);
        while let Some(mut siblings_of_sibling) = siblings_iter.fetch_next() {
            siblings_of_sibling.list.insert(new_child);
        }

        // TODO: cause problems if term_diff is big
        debug!("Term_Diff: {}", event.progress - event.mean_term);
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

        if successful_birth {
            // add the kid to the hashset of children for each parent
            if let Ok((mut children_mother, name_mother)) = query_parents.get_mut(event.mother) {
                info!("{} {} gave birth!", name_mother.first, name_mother.last);
                children_mother.list.insert(new_child);
            }
            if let Ok((mut children_father, _name_father)) = query_parents.get_mut(event.father) {
                children_father.list.insert(new_child);
            }
        } else {
            // TODO: make some fucked up shit happen
            // mum dies? baby dies? :(
        }
    }
}

pub struct ReproductionPlugin;

impl Plugin for ReproductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_pregnancy, handle_givebirth).chain());
    }
}
