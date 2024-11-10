use bevy::{prelude::*, utils::HashSet};

use crate::life::Alive;
use crate::reproduction::{ChildBearing, Pregnancy, HUMAN_PREGNANCY_LENGTH, HUMAN_PREGNANCY_STD};

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

#[derive(Bundle)]
pub struct PersonBundle {
    person: Person,
    alive: Alive,
    name: Name,
    parents: Parents,
    children: Children,
    siblings: Siblings,
}

impl PersonBundle {
    pub fn new_child(
        first: &str,
        last: &str,
        parents: HashSet<Entity>,
        siblings: HashSet<Entity>,
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
        }
    }

    fn initial_people(first: &str, last: &str) -> Self {
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
        }
    }
}

fn add_people(mut commands: Commands) {
    let jack = commands
        .spawn(PersonBundle::initial_people("Jack", "Allan"))
        .id();

    let pau = commands
        .spawn((
            PersonBundle::initial_people("Paulina", "Morales"),
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
        ))
        .id();

    let pip = commands
        .spawn(PersonBundle::new_child(
            "Pip",
            "Morales-Allan",
            [jack, pau].into(),
            [].into(),
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
    commands
        .entity(pip)
        .insert(Siblings { set: [albie].into() });

    let jacob = commands
        .spawn(PersonBundle::initial_people("Jacob", "Wilmot"))
        .id();

    let pepsi = commands
        .spawn(PersonBundle::new_child(
            "Pepsi",
            "Wilmot",
            [jacob].into(),
            [].into(),
        ))
        .id();

    commands.entity(jacob).insert(Children {
        set: [pepsi].into(),
    });
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        info!("Hello {} {}!", name.first, name.last)
    }
}

fn greet_people_with_children(
    query_parents: Query<(&Name, &Children), With<Person>>,
    query_children: Query<(&Name, &Parents), With<Person>>,
) {
    for (name, children) in &query_parents {
        info!("Hello {} {}!", name.first, name.last);

        for (child_name, _) in query_children.iter_many(&children.set) {
            info!(
                "{} {} has a child called {} {}",
                name.first, name.last, child_name.first, child_name.last,
            )
        }
    }
}

fn greet_people_with_parents(
    query_children: Query<(&Name, &Parents), With<Person>>,
    query_parents: Query<(&Name, &Children), With<Person>>,
) {
    for (name, parents) in &query_children {
        info!("Hello {} {}!", name.first, name.last);

        for (parent_name, _) in query_parents.iter_many(&parents.set) {
            info!(
                "{} {} has a parent called {} {}",
                name.first, name.last, parent_name.first, parent_name.last,
            )
        }
    }
}

fn greet_people_with_siblings(query: Query<(&Name, &Siblings), With<Person>>) {
    for (name, siblings) in &query {
        for (sibling_name, _) in query.iter_many(&siblings.set) {
            info!(
                "{} {} has a sibling: {} {}",
                name.first, name.last, sibling_name.first, sibling_name.last
            )
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
        app.add_systems(
            Update,
            ((
                update_people,
                greet_people,
                greet_people_with_children,
                greet_people_with_parents,
                greet_people_with_siblings,
            )
                .chain(),),
        );
    }
}
