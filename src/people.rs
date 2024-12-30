use bevy::{prelude::*, utils::HashSet};

use crate::age::Age;
use crate::life::Alive;
use crate::moon::MoonHouse;
use crate::reproduction::{ChildBearing, Pregnancy, HUMAN_PREGNANCY_LENGTH, HUMAN_PREGNANCY_STD};
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
    person: Person,
    alive: Alive,
    name: Name,
    parents: Parents,
    children: Children,
    siblings: Siblings,
    moon_house: AssignedMoonHouse,
    age: Age,
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

    fn initial_people(first: &str, last: &str, house: MoonHouse, age: i32) -> Self {
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

fn add_people(mut commands: Commands) {
    let jack = commands
        .spawn(PersonBundle::initial_people(
            "Jack",
            "Allan",
            MoonHouse::Death,
            12000,
        ))
        .id();

    let pau = commands
        .spawn((
            PersonBundle::initial_people("Paulina", "Morales", MoonHouse::Storm, 10555),
            ChildBearing,
            Pregnancy::new(HUMAN_PREGNANCY_LENGTH, HUMAN_PREGNANCY_STD, jack),
        ))
        .id();

    let albie = commands
        .spawn(PersonBundle::new_child(
            "Albert",
            "Morales-Allan",
            [jack, pau].into(),
            [].into(),
            MoonHouse::Light,
            293,
        ))
        .id();

    let pip = commands
        .spawn(PersonBundle::new_child(
            "Pip",
            "Morales-Allan",
            [jack, pau].into(),
            [].into(),
            MoonHouse::Wild,
            854,
        ))
        .id();

    commands.entity(jack).insert(Children {
        set: [albie, pip].into(),
    });
    commands.entity(pau).insert(Children {
        set: [albie, pip].into(),
    });

    commands
        .entity(albie)
        .insert(Siblings { set: [pip].into() });
    commands.entity(pip).insert(Siblings {
        set: [albie].into(),
    });

    let jacob = commands
        .spawn(PersonBundle::initial_people(
            "Jacob",
            "Wilmot",
            MoonHouse::Wind,
            10800,
        ))
        .id();

    let pepsi = commands
        .spawn(PersonBundle::new_child(
            "Pepsi",
            "Wilmot",
            [jacob].into(),
            [].into(),
            MoonHouse::Fire,
            7000,
        ))
        .id();

    commands.entity(jacob).insert(Children {
        set: [pepsi].into(),
    });
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

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.first == "Paulina" && name.last == "Morales" {
            name.last = "Morales-Allan".into();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, ((update_people, greet_people).chain(),).run_if(in_state(RunState::Running)));
    }
}
