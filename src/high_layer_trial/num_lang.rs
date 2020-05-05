use std::sync::Arc;

use super::super::data::music_info::beat::Beat;
use super::super::data::music_info::note::Note;
use super::super::data::music_info::phrase::Phrase;
use super::super::music_state::music_state::{MusicState, MusicStateEvent};
use super::super::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use super::super::players::player::Player;

fn parse_num_lang(s: String, octave: f32, key: f32) -> Phrase {
    let mut now: Beat = Beat::from(0);
    let length_unit: Beat = Beat::from(0.5);
    let mut phrase = Phrase::new();
    let pitch_offset: f32 = octave * 12.0 + key;

    phrase = phrase.set_repeat_length(Beat::from(s.len() as f32 / 2.0));

    for c in s.chars() {
        let pitch = match c {
            '0' => Some(47.0),
            '1' => Some(48.0),
            '2' => Some(50.0),
            '3' => Some(52.0),
            '4' => Some(53.0),
            '5' => Some(55.0),
            '6' => Some(57.0),
            '7' => Some(59.0),
            '8' => Some(60.0),
            '9' => Some(62.0),
            _ => None,
        };

        match pitch {
            Some(pitch) => {
                let note = Note {
                    pitch: pitch + pitch_offset,
                    duration: length_unit,
                    start: now,
                };
                phrase = phrase.add_note(note);
            }
            None => {}
        }

        now = now + length_unit;
    }
    phrase
}

pub fn send_num_lang(
    phrase_string: String,
    octave: f32,
    key: f32,
    phrase_name: String,
    player: Arc<dyn Player<MusicState, MusicStateEvent, WaveReader, Vec<i16>, WaveReaderEvent>>,
) -> Result<(), String> {
    player.send_event(MusicStateEvent::NewPhrase(
        phrase_name.clone(),
        parse_num_lang(phrase_string, octave, key),
    ))?;
    Ok(())
}
