use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct FrameCount {
    pub count: u32,
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
