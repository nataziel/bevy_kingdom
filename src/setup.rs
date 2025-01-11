use crate::{
    moon::MoonHouse,
    people::{Children, PersonBundle, Siblings},
    reproduction::{ChildBearing, Pregnancy, HUMAN_PREGNANCY_LENGTH, HUMAN_PREGNANCY_STD},
};
use bevy::{prelude::*, utils::HashSet};
use rand::prelude::*;

fn create_initial_people(mut commands: Commands) {
    // TODO: read the lists from file? either way, rethink how we get these names.
    // create vectors of first names
    let male_first_names = vec![
        "Tim", "Tom", "Lachlan", "Alex", "Jack", "Chris", "Harry", "Stephen", "Paul",
    ];
    let female_first_names = vec![
        "Paulina", "Alex", "Lillian", "Pia", "Tess", "Phoebe", "Kim", "Myra", "Claudia",
    ];
    // create a vector of last names
    let mut last_names = vec![
        "Allan",
        "Morales",
        "Miller",
        "Smith",
        "Baker",
        "Tanner",
        "Peddler",
        "Freeman",
        "Butcher",
        "Carpenter",
        "Farrier",
        "Primrose",
        "Stubbs",
        "McNaughton",
        "Morris",
        "Ploughman",
        "Bowyer",
        "Tinker",
    ];

    let mut rng = thread_rng();

    // create n couples to be the initial people in the kingdom
    for _ in 0..7 {
        let mut family_male_first_names = male_first_names.clone();
        family_male_first_names.shuffle(&mut rng);

        let mut family_female_first_names = female_first_names.clone();
        family_female_first_names.shuffle(&mut rng);

        let family_name = last_names.pop().unwrap();

        let father = commands
            .spawn(PersonBundle::initial_people(
                family_male_first_names.pop().unwrap(),
                family_name,
                MoonHouse::random(&mut rng),
                rng.gen_range(5844..14610),
            ))
            .id();

        let mother = commands
            .spawn((
                PersonBundle::initial_people(
                    family_female_first_names.pop().unwrap(),
                    last_names.pop().unwrap(),
                    MoonHouse::random(&mut rng),
                    rng.gen_range(5844..14610),
                ),
                ChildBearing,
            ))
            .id();

        // create a random number of children for the couple
        let mut children = HashSet::new();
        for _ in 1..rng.gen_range(0..8) {
            let child = if rand::random() {
                commands
                    .spawn(PersonBundle::new_child(
                        family_male_first_names.pop().unwrap(),
                        family_name,
                        [father, mother].into(),
                        [].into(),
                        MoonHouse::random(&mut rng),
                        0,
                    ))
                    .id()
            } else {
                commands
                    .spawn((
                        PersonBundle::new_child(
                            family_female_first_names.pop().unwrap(),
                            family_name,
                            [father, mother].into(),
                            [].into(),
                            MoonHouse::random(&mut rng),
                            0,
                        ),
                        ChildBearing,
                    ))
                    .id()
            };
            // add the child to the HashSet so we can handle them later
            children.insert(child);
        }

        // add the children to the parents
        commands.entity(father).insert(Children {
            set: children.clone(),
        });
        commands.entity(mother).insert(Children {
            set: children.clone(),
        });

        // for each child, insert their siblings
        for child in &children {
            let mut siblings = Siblings {
                set: children.clone(),
            };
            siblings.set.remove(child);
            commands.entity(*child).insert(siblings);
        }
    }
}

fn add_custom_people(mut commands: Commands) {
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
            PersonBundle::initial_people("Paulina", "Morales-Allan", MoonHouse::Storm, 10555),
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

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_custom_people, create_initial_people));
    }
}
