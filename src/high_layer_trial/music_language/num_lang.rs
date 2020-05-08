use std::sync::Arc;

use super::super::super::data::music_info::{Beat, Note, Phrase, Pitch};
use super::super::super::music_state::music_state::{MusicState, MusicStateEvent};
use super::super::super::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use super::super::super::players::player::Player;
use super::send_phrase::send_phrase;

pub fn parse_num_lang(s: String, octave: f32, key: f32) -> Phrase {
    let mut now: Beat = Beat::from(0);
    let length_unit: Beat = Beat::from(0.5);
    let mut phrase = Phrase::new();
    let pitch_offset: f32 = octave * 12.0 + key;

    phrase = phrase.set_length(Beat::from(s.len() as f32 / 2.0));

    for c in s.chars() {
        let pitch = match c {
            '0' => Some(Pitch::from(47.0)),
            '1' => Some(Pitch::from(48.0)),
            '2' => Some(Pitch::from(50.0)),
            '3' => Some(Pitch::from(52.0)),
            '4' => Some(Pitch::from(53.0)),
            '5' => Some(Pitch::from(55.0)),
            '6' => Some(Pitch::from(57.0)),
            '7' => Some(Pitch::from(59.0)),
            '8' => Some(Pitch::from(60.0)),
            '9' => Some(Pitch::from(62.0)),
            _ => None,
        };

        match pitch {
            Some(pitch) => {
                let note = Note {
                    pitch: pitch.add_f32(pitch_offset),
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
    sf2_name: Option<String>,
    vol: f32,
    pan: f32,
    player: Arc<
        dyn Player<MusicState, MusicStateEvent, WaveReader, (Vec<i16>, Vec<i16>), WaveReaderEvent>,
    >,
) -> Result<(), String> {
    send_phrase(
        parse_num_lang(phrase_string, octave, key),
        phrase_name,
        sf2_name,
        vol,
        pan,
        player,
    )?;
    Ok(())
}
