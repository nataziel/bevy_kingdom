use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;

const TRANSITION_RANGE_START: u32 = 15;
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

#[derive(Debug, Eq, PartialEq)]
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

    pub fn str(&self) -> String {
        use MoonPhase::*;
        match *self {
            New => "New".to_string(),
            WaxingCrescent => "Waxing Crescent".to_string(),
            FirstQuarter => "First Quarter".to_string(),
            WaxingGibbous => "Waxing Gibbous".to_string(),
            Full => "Full".to_string(),
            WaningGibbous => "Waning Gibbous".to_string(),
            LastQuarter => "Last Quarter".to_string(),
            WaningCrescent => "Waning Crescent".to_string(),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
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

fn handle_house(moon: &mut Moon) {
    let mut rng = thread_rng();

    let mut transition_value: u32 = rng.gen_range(0..moon.transition_range);

    if moon.phase == MoonPhase::New {
        transition_value += 5
    }

    if transition_value >= moon.transition_threshold {
        if (transition_value - 5 < moon.transition_threshold) & (moon.phase == MoonPhase::New) {
            info!("House transitioned early due to the New Moon!")
        }
        moon.transition_range = TRANSITION_RANGE_START;
        moon.house = transition_moon_house(&mut rng, &mut moon.house_weights);

        info!("Moon transitioned to House {:?}", moon.house)
    } else {
        moon.transition_range += 1;
    };
}

fn transition_moon_house(rng: &mut ThreadRng, weights: &mut HashMap<MoonHouse, u32>) -> MoonHouse {
    // Turn weights into a collection we can use choose_weighted on
    let weights_collection: Vec<(MoonHouse, u32)> = weights.clone().into_iter().collect();

    // choose the new house based on the weights
    let new_house: MoonHouse = weights_collection
        .choose_weighted(rng, |item| item.1)
        .unwrap()
        .0
        .clone();

    // Set the weight of the new house to 1, increase the weight of each other house by 1
    for (house, weight) in weights.iter_mut() {
        if *house == new_house {
            *weight = 1;
        } else {
            *weight += 1
        };
    }

    return new_house;
}

pub struct MoonPlugin;

impl Plugin for MoonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_moon);
        app.add_systems(Update, handle_moon);
    }
}

fn add_moon(mut commands: Commands) {
    // Construct weights map here because can't do it as a const
    use MoonHouse::*;
    let mut house_weights_map: HashMap<MoonHouse, u32> = HashMap::new();
    house_weights_map.insert(Dark, 1);
    house_weights_map.insert(Light, 1);
    house_weights_map.insert(Fire, 1);
    house_weights_map.insert(Water, 1);
    house_weights_map.insert(Wind, 1);
    house_weights_map.insert(Earth, 1);
    house_weights_map.insert(Death, 1);
    house_weights_map.insert(Storm, 1);

    commands.spawn(Moon::new(
        MoonPhase::WaningCrescent,
        MoonHouse::Dark,
        TRANSITION_RANGE_START,
        TRANSITION_THRESHOLD,
        house_weights_map,
    ));
}

fn handle_moon(mut query: Query<&mut Moon>) {
    let mut moon = query.single_mut();

    moon.phase = moon.phase.next();

    handle_house(&mut moon);

    info!("{} Moon in High House {:?}", moon.phase.str(), moon.house);
}
