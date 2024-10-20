use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;

const TRANSITION_RANGE: u32 = 29;
const TRANSITION_THRESHOLD: u32 = 30;

#[derive(Component, Debug)]
pub struct Moon {
    pub phase: MoonPhase,
    pub house: MoonHouse,
    transition_range: u32,
    transition_threshold: u32,
    house_weights: HashMap<MoonHouse, u32>,
}

impl Moon {
    pub fn new(
        phase: MoonPhase,
        house: MoonHouse,
        transition_range: u32,
        transition_threshold: u32,
        house_weights: HashMap<MoonHouse, u32>,
    ) -> Self {
        Self {
            phase,
            house,
            transition_range,
            transition_threshold,
            house_weights,
        }
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

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum MoonHouse {
    Dark,
    Light,
    Fire,
    Water,
    Wind,
    Earth,
    Death,
    Storm,
}

impl MoonHouse {
    pub fn next(&self) -> Self {
        use MoonHouse::*;
        match *self {
            Dark => Light,
            Light => Fire,
            Fire => Water,
            Water => Wind,
            Wind => Earth,
            Earth => Death,
            Death => Storm,
            Storm => Dark,
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
    use MoonHouse::*;
    let mut house_weights_map: HashMap<MoonHouse, u32> = HashMap::new();
    house_weights_map.insert(Dark, 100);
    house_weights_map.insert(Light, 100);
    house_weights_map.insert(Fire, 100);
    house_weights_map.insert(Water, 100);
    house_weights_map.insert(Wind, 100);
    house_weights_map.insert(Earth, 100);
    house_weights_map.insert(Death, 100);
    house_weights_map.insert(Storm, 100);

    commands.spawn(Moon::new(
        MoonPhase::WaningCrescent,
        MoonHouse::Dark,
        TRANSITION_RANGE,
        TRANSITION_THRESHOLD,
        house_weights_map,
    ));
}

fn handle_moon(mut query: Query<&mut Moon>) {
    let mut moon = query.single_mut();

    moon.phase = moon.phase.next();
    println!("Moon phase: {:?}", moon.phase);

    let mut rng = thread_rng();
    let transition_value: u32 = rng.gen_range(0..moon.transition_range);

    if transition_value >= moon.transition_threshold {
        moon.transition_range = 30;
        moon.house = moon.house.next();

        println!("Moon transitioned to house {:?}", moon.house)
    } else {
        moon.transition_range += 1;
    };

    println!("{:?}", moon)
}
