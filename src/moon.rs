use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::distributions::Standard;
use rand::prelude::*;
use std::fmt;

use crate::life::Alive;
use crate::people::{AssignedMoonHouse, Name};
use crate::state::RunState;

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
        use MoonPhase::{
            FirstQuarter, Full, LastQuarter, New, WaningCrescent, WaningGibbous, WaxingCrescent,
            WaxingGibbous,
        };
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
        use MoonPhase::{
            FirstQuarter, Full, LastQuarter, New, WaningCrescent, WaningGibbous, WaxingCrescent,
            WaxingGibbous,
        };
        match *self {
            New => "New".into(),
            WaxingCrescent => "Waxing Crescent".into(),
            FirstQuarter => "First Quarter".into(),
            WaxingGibbous => "Waxing Gibbous".into(),
            Full => "Full".into(),
            WaningGibbous => "Waning Gibbous".into(),
            LastQuarter => "Last Quarter".into(),
            WaningCrescent => "Waning Crescent".into(),
        }
    }
}

impl fmt::Display for MoonPhase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
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
    Dream,
    Wild,
}

impl MoonHouse {
    pub fn str(&self) -> String {
        use MoonHouse::{Dark, Death, Dream, Earth, Fire, Light, Storm, Water, Wild, Wind};
        match *self {
            Dark => "Dark".into(),
            Light => "Light".into(),
            Fire => "Fire".into(),
            Water => "Water".into(),
            Wind => "Wind".into(),
            Earth => "Earth".into(),
            Death => "Death".into(),
            Storm => "Storm".into(),
            Dream => "Dream".into(),
            Wild => "Wild".into(),
        }
    }
}

impl Distribution<MoonHouse> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoonHouse {
        use MoonHouse::{Dark, Death, Dream, Earth, Fire, Light, Storm, Water, Wild, Wind};
        match rng.gen_range(0..10) {
            0 => Dark,
            1 => Death,
            2 => Dream,
            3 => Earth,
            4 => Fire,
            5 => Light,
            6 => Storm,
            7 => Water,
            8 => Wild,
            _ => Wind,
        }
    }
}

impl fmt::Display for MoonHouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

fn handle_house(moon: &mut Moon) -> bool {
    let mut rng = thread_rng();

    let mut transition_value: u32 = rng.gen_range(0..moon.transition_range);

    // Bonus to transition if it's a New Moon
    if moon.phase == MoonPhase::New {
        transition_value += 5;
    }

    if transition_value >= moon.transition_threshold {
        if (transition_value - 5 < moon.transition_threshold) & (moon.phase == MoonPhase::New) {
            info!("House transitioned early due to the New Moon!");
        }
        moon.transition_range = TRANSITION_RANGE_START;
        moon.house = transition_moon_house(&mut rng, &mut moon.house_weights);

        info!("Moon transitioned to House {}", moon.house);
        true
    } else {
        moon.transition_range += 1;
        false
    }
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
            *weight += 1;
        };
    }

    new_house
}

/// system resource for one-shot
#[derive(Resource, Debug)]
struct ExaltSystem(SystemId);

/// What the resource returns when initialised from the world
impl FromWorld for ExaltSystem {
    fn from_world(world: &mut World) -> Self {
        let system_id = world.register_system(exalt_house_members);

        ExaltSystem(system_id)
    }
}

/// Basic implementation of a one-shot system to see how they work
fn exalt_house_members(
    person_query: Query<(&Name, &AssignedMoonHouse), With<Alive>>,
    moon_query: Query<&Moon>,
) {
    let moon = moon_query.get_single().unwrap();

    for (name, house) in &person_query {
        if house.house == moon.house {
            info!("{} exalts {} {}", moon.house, name.first, name.last);
        }
    }
}

pub struct MoonPlugin;

impl Plugin for MoonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_moon);
        app.add_systems(Update, handle_moon.run_if(in_state(RunState::Running)));
        app.init_resource::<ExaltSystem>(); // for the exalt one-shot
    }
}

fn add_moon(mut commands: Commands) {
    // Construct weights map here because can't do it as a const
    use MoonHouse::{Dark, Death, Dream, Earth, Fire, Light, Storm, Water, Wild, Wind};
    let mut house_weights_map: HashMap<MoonHouse, u32> = HashMap::new();
    house_weights_map.insert(Dark, 1);
    house_weights_map.insert(Light, 1);
    house_weights_map.insert(Fire, 1);
    house_weights_map.insert(Water, 1);
    house_weights_map.insert(Wind, 1);
    house_weights_map.insert(Earth, 1);
    house_weights_map.insert(Death, 1);
    house_weights_map.insert(Storm, 1);
    house_weights_map.insert(Dream, 1);
    house_weights_map.insert(Wild, 1);

    commands.spawn(Moon::new(
        MoonPhase::WaningCrescent,
        MoonHouse::Dark,
        TRANSITION_RANGE_START,
        TRANSITION_THRESHOLD,
        house_weights_map,
    ));
}

fn handle_moon(
    mut query: Query<&mut Moon>,
    mut commands: Commands,
    exalt_system: Res<ExaltSystem>,
) {
    let mut moon = query.single_mut();

    moon.phase = moon.phase.next();

    let house_transition = handle_house(&mut moon);

    if house_transition {
        commands.run_system(exalt_system.0);
    }

    info!("{} Moon in High House {}", moon.phase, moon.house);
}
