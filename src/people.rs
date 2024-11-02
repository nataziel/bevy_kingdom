use bevy::prelude::*;

use crate::reproduction::{ChildBearing, Pregnancy, HUMAN_PREGNANCY_LENGTH};

#[derive(Component, Debug)]
pub struct Person;

#[derive(Component, Debug)]
pub struct Name {
    pub first: String,
    pub last: String,
}

#[derive(Component, Debug)]
pub struct Parents {
    pub list: Vec<Entity>,
}

#[derive(Component, Debug)]
pub struct Children {
    pub list: Vec<Entity>,
}

fn hello_world() {
    info!("Hello world!");
}

fn add_people(mut commands: Commands) {
    let jack = commands
        .spawn((
            Person,
            Name {
                first: "Jack".to_string(),
                last: "Allan".to_string(),
            },
        ))
        .id();

    let pau = commands
        .spawn((
            Person,
            Name {
                first: "Paulina".to_string(),
                last: "Morales".to_string(),
            },
            ChildBearing,
            Pregnancy::new(HUMAN_PREGNANCY_LENGTH, jack),
        ))
        .id();

    let albie = commands
        .spawn((
            Person,
            Name {
                first: "Albert".to_string(),
                last: "Morales-Allan".to_string(),
            },
            Parents {
                list: vec![jack, pau],
            },
        ))
        .id();

    let pip = commands
        .spawn((
            Person,
            Name {
                first: "Pip".to_string(),
                last: "Morales-Allan".to_string(),
            },
            Parents {
                list: vec![jack, pau],
            },
        ))
        .id();

    commands.entity(jack).insert(Children {
        list: vec![albie, pip],
    });
    commands.entity(pau).insert(Children {
        list: vec![albie, pip],
    });

    let jacob = commands
        .spawn((
            Person,
            Name {
                first: "Jacob".to_string(),
                last: "Wilmot".to_string(),
            },
        ))
        .id();

    let pepsi = commands
        .spawn((
            Person,
            Name {
                first: "Pepsi".to_string(),
                last: "Wilmot".to_string(),
            },
            Parents { list: vec![jacob] },
        ))
        .id();

    commands
        .entity(jacob)
        .insert(Children { list: vec![pepsi] });
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

        for (child_name, _) in query_children.iter_many(&children.list) {
            info!(
                "{} {} has a child called {} {}",
                name.first, name.last, child_name.first, child_name.last,
            )
        }
    }
}

fn greet_people_with_parents(
    query_parents: Query<(&Name, &Children), With<Person>>,
    query_children: Query<(&Name, &Parents), With<Person>>,
) {
    for (name, parents) in &query_children {
        info!("Hello {} {}!", name.first, name.last);

        for (parent_name, _) in query_parents.iter_many(&parents.list) {
            info!(
                "{} {} has a parent called {} {}",
                name.first, name.last, parent_name.first, parent_name.last,
            )
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.first == "Paulina" && name.last == "Morales" {
            name.last = "Morales-Allan".to_string();
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
            (
                hello_world,
                (
                    update_people,
                    greet_people,
                    greet_people_with_children,
                    greet_people_with_parents,
                )
                    .chain(),
            ),
        );
    }
}
