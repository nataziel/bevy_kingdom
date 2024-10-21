use bevy::prelude::*;

#[derive(Component, Debug)]
struct Weather {
    temp: i32,
    rainfall: i32,
    wind: f32,
}
