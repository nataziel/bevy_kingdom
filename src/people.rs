use bevy::{prelude::*, utils::HashSet};

use crate::age::Age;
use crate::life::Alive;
use crate::moon::MoonHouse;
use crate::state::RunState;

#[derive(Component, Debug)]
pub struct Person;

#[derive(Component, Debug)]
pub struct Name {
    pub first: String,
    pub last: String,
}

#[derive(Component, Debug)]
pub struct Parents {
    pub set: HashSet<Entity>,
}

#[derive(Component, Debug)]
pub struct Children {
    pub set: HashSet<Entity>,
}

#[derive(Component, Debug)]
pub struct Siblings {
    pub set: HashSet<Entity>,
}

#[derive(Component, Debug)]
pub struct AssignedMoonHouse {
    pub house: MoonHouse,
}

#[derive(Bundle)]
pub struct PersonBundle {
    pub person: Person,
    pub alive: Alive,
    pub name: Name,
    pub parents: Parents,
    pub children: Children,
    pub siblings: Siblings,
    pub moon_house: AssignedMoonHouse,
    pub age: Age,
}

impl PersonBundle {
    pub fn new_child(
        first: &str,
        last: &str,
        parents: HashSet<Entity>,
        siblings: HashSet<Entity>,
        house: MoonHouse,
        age: i32,
    ) -> Self {
        PersonBundle {
            person: Person,
            alive: Alive,
            name: Name {
                first: first.into(),
                last: last.into(),
            },
            parents: Parents { set: parents },
            children: Children {
                set: HashSet::new(),
            },
            siblings: Siblings { set: siblings },
            moon_house: AssignedMoonHouse { house },
            age: Age::new(age),
        }
    }

    pub fn initial_people(first: &str, last: &str, house: MoonHouse, age: i32) -> Self {
        PersonBundle {
            person: Person,
            alive: Alive,
            name: Name {
                first: first.into(),
                last: last.into(),
            },
            parents: Parents {
                set: HashSet::new(),
            },
            children: Children {
                set: HashSet::new(),
            },
            siblings: Siblings {
                set: HashSet::new(),
            },
            moon_house: AssignedMoonHouse { house },
            age: Age::new(age),
        }
    }
}

fn greet_people(
    query_people: Query<
        (
            &Name,
            &Age,
            &Children,
            &Parents,
            &Siblings,
            &AssignedMoonHouse,
        ),
        With<Person>,
    >,
) {
    for (name, age, children, parents, siblings, assigned_house) in &query_people {
        debug!("Hello {} {}({})!", name.first, name.last, age);
        debug!(
            "{} {} is favoured by High House {}",
            name.first, name.last, assigned_house.house
        );

        for (child_name, _, _, _, _, _) in query_people.iter_many(&children.set) {
            debug!(
                "{} {} has a child called {} {}",
                name.first, name.last, child_name.first, child_name.last,
            );
        }

        for (parent_name, _, _, _, _, _) in query_people.iter_many(&parents.set) {
            debug!(
                "{} {} has a parent called {} {}",
                name.first, name.last, parent_name.first, parent_name.last,
            );
        }

        for (sibling_name, _, _, _, _, _) in query_people.iter_many(&siblings.set) {
            debug!(
                "{} {} has a sibling: {} {}",
                name.first, name.last, sibling_name.first, sibling_name.last
            );
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, greet_people.run_if(in_state(RunState::Running)));
    }
}
