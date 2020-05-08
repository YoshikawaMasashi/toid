use std::sync::Arc;

use super::super::super::data::music_info::phrase::Phrase;
use super::super::super::music_state::music_state::{MusicState, MusicStateEvent};
use super::super::super::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use super::super::super::players::player::Player;

pub fn send_phrase(
    phrase: Phrase,
    phrase_name: String,
    player: Arc<
        dyn Player<MusicState, MusicStateEvent, WaveReader, (Vec<i16>, Vec<i16>), WaveReaderEvent>,
    >,
) -> Result<(), String> {
    player.send_event(MusicStateEvent::NewPhrase(phrase_name.clone(), phrase))?;
    Ok(())
}
