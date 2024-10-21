use bevy::prelude::*;

#[derive(Component, Debug)]
struct Person;

#[derive(Component, Debug)]
struct Name {
    first: String,
    last: String,
}

fn hello_world() {
    info!("Hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((
        Person,
        Name {
            first: "Jack".to_string(),
            last: "Allan".to_string(),
        },
    ));
    commands.spawn((
        Person,
        Name {
            first: "Paulina".to_string(),
            last: "Morales".to_string(),
        },
    ));
    commands.spawn((
        Person,
        Name {
            first: "Albert".to_string(),
            last: "Morales-Allan".to_string(),
        },
    ));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        info!("Hello {} {}!", name.first, name.last)
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
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}
