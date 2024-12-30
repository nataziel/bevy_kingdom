use bevy::prelude::*;
use crate::frames::FrameCount;

#[derive(Default, States, Clone, Eq, PartialEq, Hash, Debug)]
pub enum RunState {
    #[default]
    Running,
    Paused,
}

fn toggle_pause_state(framecount: Res<FrameCount>, state: Res<State<RunState>>, mut next_state: ResMut<NextState<RunState>>) {
    // pause and unpause the simulation every 120 frames
    if (framecount.count % 120) == 0 {
    match state.get() {
        RunState::Running => next_state.set(RunState::Paused),
        RunState::Paused => next_state.set(RunState::Running),
    }
}
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RunState>();
        app.add_systems(Update, toggle_pause_state);
    }
}
