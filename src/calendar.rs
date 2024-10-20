use bevy::prelude::*;
use bevy::utils::HashMap;

const YEAR_LENGTH: u32 = 365;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum MonthName {
    Messidor,
    Termidor,
    Fructidor,
    Brumaire,
    Frimaire,
    Nivose,
    Pluviose,
    Ventose,
    Germinal,
    Floreal,
    Prairial,
    SansCullotides,
}

impl MonthName {
    pub fn next(&self) -> Self {
        use MonthName::*;
        match *self {
            // todo: fix month lengths/transitions??
            Messidor => Termidor,
            Termidor => Fructidor,
            Fructidor => Brumaire,
            Brumaire => Frimaire,
            Frimaire => Nivose,
            Nivose => Pluviose,
            Pluviose => Ventose,
            Ventose => Germinal,
            Germinal => Floreal,
            Floreal => Prairial,
            Prairial => SansCullotides,
            SansCullotides => Messidor,
        }
    }
}

#[derive(Component, Debug)]
pub struct Calendar {
    year: u32,
    year_day: u32,
    month: MonthName,
    month_day: u32,
    year_length: u32,
    month_map: HashMap<MonthName, u32>,
}

impl Calendar {
    fn new(
        year: u32,
        year_day: u32,
        month: MonthName,
        month_day: u32,
        year_length: u32,
        month_map: HashMap<MonthName, u32>,
    ) -> Self {
        Self {
            year,
            year_day,
            month,
            month_day,
            year_length,
            month_map,
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
    use MonthName::*;
    let mut month_map: HashMap<MonthName, u32> = HashMap::new();
    month_map.insert(Messidor, 30);
    month_map.insert(Termidor, 30);
    month_map.insert(Fructidor, 30);
    month_map.insert(Brumaire, 30);
    month_map.insert(Frimaire, 30);
    month_map.insert(Nivose, 30);
    month_map.insert(Pluviose, 30);
    month_map.insert(Ventose, 30);
    month_map.insert(Germinal, 30);
    month_map.insert(Floreal, 30);
    month_map.insert(Prairial, 30);
    month_map.insert(SansCullotides, 5);

    let calendar = Calendar::new(0, 0, Messidor, 0, YEAR_LENGTH, month_map);
    println!("{:?}", &calendar);

    commands.spawn(calendar);
}

fn advance_date(mut query: Query<&mut Calendar>) {
    let mut calendar = query.single_mut();

    //handle months
    let current_month: MonthName = calendar.month.clone();
    calendar.month_day += 1;

    if calendar.month_day > calendar.month_map[&current_month] {
        calendar.month = calendar.month.next();
        calendar.month_day = 1;
        println!(
            "Month {:?} transitioned to next month {:?}",
            current_month, calendar.month
        )
    }

    //handler years
    calendar.year_day += 1;

    if calendar.year_day > calendar.year_length {
        calendar.year += 1;
        calendar.year_day = 1
    }

    println!("{:?}", calendar)
}
