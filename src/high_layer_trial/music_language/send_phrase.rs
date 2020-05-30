use std::sync::Arc;

use super::super::super::data::music_info::{
    Beat, Instrument, Phrase, SamplePhrase, SampleTrack, Track,
};
use super::super::super::music_state::states::{MusicState, MusicStateEvent, SectionStateEvent};
use super::super::super::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use super::super::super::players::player::Player;

pub fn send_phrase(
    phrase: Phrase,
    section_beat: Beat,
    track_name: String,
    instrument: Instrument,
    vol: f32,
    pan: f32,
    player: Arc<
        dyn Player<MusicState, MusicStateEvent, WaveReader, (Vec<i16>, Vec<i16>), WaveReaderEvent>,
    >,
) -> Result<(), String> {
    let track = Track {
        phrase,
        instrument,
        vol,
        pan,
    };
    player.send_event(MusicStateEvent::SectionStateEvent(
        section_beat,
        SectionStateEvent::NewTrack(track_name.clone(), track),
    ))?;
    Ok(())
}

pub fn send_sample_phrase(
    phrase: SamplePhrase,
    section_beat: Beat,
    track_name: String,
    sample_name: String,
    vol: f32,
    pan: f32,
    player: Arc<
        dyn Player<MusicState, MusicStateEvent, WaveReader, (Vec<i16>, Vec<i16>), WaveReaderEvent>,
    >,
) -> Result<(), String> {
    let track = SampleTrack {
        phrase,
        sample_name,
        vol,
        pan,
    };
    player.send_event(MusicStateEvent::SectionStateEvent(
        section_beat,
        SectionStateEvent::NewSampleTrack(track_name.clone(), track),
    ))?;
    Ok(())
}
