use bevy::prelude::*;

#[derive(Resource)]
struct FrameCount {
    count: u32,
}

impl Default for FrameCount {
    fn default() -> Self {
        FrameCount { count: 0 }
    }
}

fn handle_framecount(mut framecount: ResMut<FrameCount>) {
    framecount.count += 1;
}

pub struct FramePlugin;

impl Plugin for FramePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameCount>();
        app.add_systems(Update, handle_framecount);
    }
}
