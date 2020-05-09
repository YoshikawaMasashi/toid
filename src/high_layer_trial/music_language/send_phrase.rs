use std::sync::Arc;

use super::super::super::data::music_info::{Beat, Phrase, Track};
use super::super::super::music_state::states::{MusicState, MusicStateEvent, SectionStateEvent};
use super::super::super::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use super::super::super::players::player::Player;

pub fn send_phrase(
    phrase: Phrase,
    section_beat: Beat,
    track_name: String,
    sf2_name: Option<String>,
    vol: f32,
    pan: f32,
    player: Arc<
        dyn Player<MusicState, MusicStateEvent, WaveReader, (Vec<i16>, Vec<i16>), WaveReaderEvent>,
    >,
) -> Result<(), String> {
    let track = Track {
        phrase,
        sf2_name,
        vol,
        pan,
    };
    player.send_event(MusicStateEvent::SectionStateEvent(
        section_beat,
        SectionStateEvent::NewTrack(track_name.clone(), track),
    ))?;
    Ok(())
}
