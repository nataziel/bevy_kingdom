use bevy::prelude::*;

use crate::{life::Alive, state::RunState};

#[derive(Component, Debug)]
pub struct Age {
    days: i32,
}

impl Age {
    pub fn new(days: i32) -> Self {
        Age { days }
    }
}

impl std::fmt::Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let years = self.days / 365;
        let days = self.days % 365;
        write!(f, "{years} Years, {days} Days",)
    }
}

fn handle_age(mut query: Query<&mut Age, With<Alive>>) {
    for mut age in &mut query {
        age.days += 1
    }
}

pub struct AgePlugin;

impl Plugin for AgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_age.run_if(in_state(RunState::Running)));
    }
}
