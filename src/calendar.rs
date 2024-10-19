use bevy::prelude::*;

const YEAR_LENGTH: i32 = 365;

#[derive(Component, Debug)]
pub struct Calendar {
    year: i32,
    day: i32,
    year_length: i32,
}

impl Calendar {
    fn new(year: i32, day: i32, year_length: i32) -> Self {
        Self {
            year,
            day,
            year_length,
        }
    }
}

pub struct DatePlugin;

impl Plugin for DatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_calendar);
        app.add_systems(Update, advance_date);
    }
}

fn add_calendar(mut commands: Commands) {
    let calendar = Calendar::new(0, -1, YEAR_LENGTH);
    println!("{:?}", &calendar);

    commands.spawn(calendar);
}

fn advance_date(mut query: Query<&mut Calendar>) {
    let mut calendar = query.single_mut();

    calendar.day += 1;

    if calendar.day >= calendar.year_length {
        calendar.year += 1;
        calendar.day = 0
    }

    println!("{:?}", calendar)
}
