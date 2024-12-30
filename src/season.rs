use crate::{
    calendar::{Calendar, MonthName},
    state::RunState,
};
use bevy::prelude::*;

#[derive(Component, Debug)]
enum Season {
    Summer,
    Autumn,
    Winter,
    Spring,
}

impl Season {
    pub fn next(&self) -> Self {
        use Season::{Autumn, Spring, Summer, Winter};
        match *self {
            Summer => Autumn,
            Autumn => Winter,
            Winter => Spring,
            Spring => Summer,
        }
    }

    pub fn month_to_season(month: MonthName) -> Self {
        use crate::calendar::MonthName::{
            Brumaire, Floreal, Frimaire, Fructidor, Germinal, Messidor, Nivose, Pluviose, Prairial,
            SansCulottides, Termidor, Vendemiaire, Ventose,
        };
        use Season::{Autumn, Spring, Summer, Winter};
        match month {
            Messidor | Termidor | Fructidor => Summer,
            Vendemiaire | Brumaire | Frimaire => Autumn,
            Nivose | Pluviose | Ventose => Winter,
            Germinal | Floreal | Prairial | SansCulottides => Spring,
        }
    }
}

pub struct SeasonPlugin;

impl Plugin for SeasonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_season);
        app.add_systems(Update, handle_season.run_if(in_state(RunState::Running)));
    }
}

fn add_season(mut commands: Commands) {
    commands.spawn(Season::Summer);
}

fn handle_season(mut season_query: Query<&mut Season>, calendar_query: Query<&Calendar>) {
    let calendar = calendar_query.single();
    let mut season = season_query.single_mut();

    *season = Season::month_to_season(calendar.month);

    debug!("{:?}", *season);
}
