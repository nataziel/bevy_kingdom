use std::fmt;

use bevy::prelude::*;
use bevy::utils::HashMap;

const YEAR_LENGTH: u32 = 365;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum MonthName {
    Messidor,
    Termidor,
    Fructidor,
    Vendemiaire,
    Brumaire,
    Frimaire,
    Nivose,
    Pluviose,
    Ventose,
    Germinal,
    Floreal,
    Prairial,
    SansCulottides,
}

impl fmt::Display for MonthName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MonthName::*;
        let out_string: String = match *self {
            Messidor => "Messidor".into(),
            Termidor => "Termidor".into(),
            Fructidor => "Fructidor".into(),
            Vendemiaire => "Vendemiaire".into(),
            Brumaire => "Brumaire".into(),
            Frimaire => "Frimaire".into(),
            Nivose => "Nivose".into(),
            Pluviose => "Pluviose".into(),
            Ventose => "Ventose".into(),
            Germinal => "Germinal".into(),
            Floreal => "Floreal".into(),
            Prairial => "Prairial".into(),
            SansCulottides => "Sans Culottides".into(),
        };
        write!(f, "{}", out_string)
    }
}

impl MonthName {
    pub fn next(&self) -> Self {
        use MonthName::*;
        match *self {
            Messidor => Termidor,
            Termidor => Fructidor,
            Fructidor => Vendemiaire,
            Vendemiaire => Brumaire,
            Brumaire => Frimaire,
            Frimaire => Nivose,
            Nivose => Pluviose,
            Pluviose => Ventose,
            Ventose => Germinal,
            Germinal => Floreal,
            Floreal => Prairial,
            Prairial => SansCulottides,
            SansCulottides => Messidor,
        }
    }
}

#[derive(Component, Debug)]
pub struct Calendar {
    year: u32,
    year_day: u32,
    pub month: MonthName,
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

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{month_day} {month_name}, Year {year}",
            month_day = self.month_day,
            month_name = self.month,
            year = self.year
        )
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
    month_map.insert(Vendemiaire, 30);
    month_map.insert(Brumaire, 30);
    month_map.insert(Frimaire, 30);
    month_map.insert(Nivose, 30);
    month_map.insert(Pluviose, 30);
    month_map.insert(Ventose, 30);
    month_map.insert(Germinal, 30);
    month_map.insert(Floreal, 30);
    month_map.insert(Prairial, 30);
    month_map.insert(SansCulottides, 5);

    let calendar = Calendar::new(0, 0, Messidor, 0, YEAR_LENGTH, month_map);
    debug!("{:?}", &calendar);

    commands.spawn(calendar);
}

fn advance_date(mut query: Query<&mut Calendar>) {
    let mut calendar = query.single_mut();

    handle_months(&mut calendar);

    handle_years(&mut calendar);

    debug!("{:?}", calendar);
    info!("{}", *calendar)
}

fn handle_months(calendar: &mut Mut<'_, Calendar>) {
    let current_month: MonthName = calendar.month;
    calendar.month_day += 1;

    if calendar.month_day > calendar.month_map[&current_month] {
        calendar.month = calendar.month.next();
        calendar.month_day = 1;
        info!(
            "Month {} ended, transitioned to next month {}",
            current_month, calendar.month
        )
    }
}

fn handle_years(calendar: &mut Mut<'_, Calendar>) {
    calendar.year_day += 1;

    if calendar.year_day > calendar.year_length {
        calendar.year += 1;
        calendar.year_day = 1;

        info!(
            "Year {} ended, transitioned to Year {}",
            calendar.year - 1,
            calendar.year
        )
    }
}
