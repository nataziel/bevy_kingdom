use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Moon {
    pub phase: MoonPhase,
}

impl Moon {
    pub fn new(phase: MoonPhase) -> Self {
        Self { phase }
    }
}

#[derive(Debug)]
pub enum MoonPhase {
    New,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    Full,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

impl MoonPhase {
    pub fn next(&self) -> Self {
        use MoonPhase::*;
        match *self {
            New => WaxingCrescent,
            WaxingCrescent => FirstQuarter,
            FirstQuarter => WaxingGibbous,
            WaxingGibbous => Full,
            Full => WaningGibbous,
            WaningGibbous => LastQuarter,
            LastQuarter => WaningCrescent,
            WaningCrescent => New,
        }
    }
}

pub struct MoonPlugin;

impl Plugin for MoonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_moon);
        app.add_systems(Update, handle_moon);
    }
}

fn add_moon(mut commands: Commands) {
    commands.spawn(Moon::new(MoonPhase::WaningCrescent));
}

fn handle_moon(mut query: Query<&mut Moon>) {
    let mut moon = query.single_mut();

    moon.phase = moon.phase.next();

    println!("{:?}", moon)
}
