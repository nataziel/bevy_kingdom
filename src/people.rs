use bevy::prelude::*;

#[derive(Component, Debug)]
struct Person;

#[derive(Component, Debug)]
struct Name {
    first: String,
    last: String,
}

#[derive(Component, Debug)]
struct Parents {
    list: Vec<Entity>,
}

#[derive(Component, Debug)]
struct Children {
    list: Vec<Entity>,
}

#[derive(Component, Debug)]
struct ChildBearing;

const HUMAN_PREGNANCY_LENGTH: i32 = 266;

#[derive(Component, Debug)]
struct Pregnancy {
    mean_term: i32,
    progress: i32,
    father: Entity,
}

impl Pregnancy {
    fn new(mean_term: i32, father: Entity) -> Self {
        Pregnancy {
            mean_term,
            progress: 0,
            father,
        }
    }
}

#[derive(Event, Debug)]
pub struct GiveBirthEvent(Entity);

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

fn handle_pregnancy(
    mut ev_give_birth: EventWriter<GiveBirthEvent>,
    mut query: Query<(Entity, &Name, &mut Pregnancy), (With<Person>, With<ChildBearing>)>,
) {
    for (entity, name, mut pregnancy) in &mut query {
        pregnancy.progress += 1;
        info!(
            "{} {} is pregnant, {}/{}",
            name.first, name.last, pregnancy.progress, pregnancy.mean_term
        );
        if pregnancy.progress >= pregnancy.mean_term {
            ev_give_birth.send(GiveBirthEvent(entity));
            // TODO: remove pregnancy trait?
            pregnancy.progress = 0;
        }
    }
}

fn debug_givebirth(mut ev_give_birth: EventReader<GiveBirthEvent>) {
    // TODO: make this create a new entity, set parents etc.
    // TODO: should probably move the pregnancy stuff to a separate file
    for event in ev_give_birth.read() {
        debug!("Entity {:?} gave birth!", event.0)
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
                handle_pregnancy,
                debug_givebirth,
            ),
        );
    }
}
