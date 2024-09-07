use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioApp, AudioChannel, AudioControl, AudioPlugin, AudioSource};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin);
        app.add_audio_channel::<BackGroundMusic>();
        app.add_systems(Startup, play_background_audio);
        app.add_systems(Update, play_sound_effect);
        app.add_event::<PlaySFXEvent>();
    }
}

#[derive(Event)]
pub struct PlaySFXEvent {
    pub sound_effect: Handle<AudioSource>,
}

#[derive(Resource)]
struct BackGroundMusic;

fn play_background_audio(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    audio.play(asset_server.load("bgm.mp3")).looped();
}

fn play_sound_effect(mut event_reader: EventReader<PlaySFXEvent>, audio: Res<Audio>) {
    for event in event_reader.read() {
        audio.play(event.sound_effect.clone());
    }
}
