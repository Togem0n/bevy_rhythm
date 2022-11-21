use crate::types::SongConfig;
use bevy::prelude::*;
use crate::consts::*;

fn start_song(audio: Res<Audio>, time: Res<Time>, asset_serve: Res<AssetServer>) {
    // Song starts 3 seconds after real time
    let secs = time.elapsed_seconds_f64();
    let secs_last = secs - time.delta_seconds_f64();
    let music = asset_serve.load("songs/audio.ogg");
    audio.play(music);
    println!("nmsl");
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(start_song);
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Game)
            .with_system(start_song)
        );
    }
}