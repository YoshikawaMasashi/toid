use std::sync::Arc;

use super::super::music_state::beat::Beat;
use super::super::music_state::melody_state::MelodyStateEvent;
use super::super::music_state::melody_state::NoteInfo;
use super::super::music_state::music_state::{MusicState, MusicStateEvent};
use super::super::players::player::Player;

fn parse_num_lang(s: String) -> Vec<NoteInfo> {
    let mut ret: Vec<NoteInfo> = vec![];
    let mut now: Beat = Beat::from(0);
    let length_unit: Beat = Beat::from(0.5);

    for c in s.chars() {
        match c {
            '1' => ret.push(NoteInfo {
                pitch: 48.0,
                duration: length_unit,
                start: now,
            }),
            '2' => ret.push(NoteInfo {
                pitch: 50.0,
                duration: length_unit,
                start: now,
            }),
            '3' => ret.push(NoteInfo {
                pitch: 52.0,
                duration: length_unit,
                start: now,
            }),
            '4' => ret.push(NoteInfo {
                pitch: 53.0,
                duration: length_unit,
                start: now,
            }),
            '5' => ret.push(NoteInfo {
                pitch: 55.0,
                duration: length_unit,
                start: now,
            }),
            '6' => ret.push(NoteInfo {
                pitch: 57.0,
                duration: length_unit,
                start: now,
            }),
            '7' => ret.push(NoteInfo {
                pitch: 59.0,
                duration: length_unit,
                start: now,
            }),
            _ => {}
        }
        now = now + length_unit;
    }
    ret
}

pub fn send_num_lang(
    melody_string: String,
    melody_name: String,
    player: Arc<dyn Player<MusicState, MusicStateEvent>>,
) {
    player.send_event(MusicStateEvent::NewMelody(melody_name.clone()));
    let note_infos = parse_num_lang(melody_string);
    for &note_info in note_infos.iter() {
        player.send_event(MusicStateEvent::MelodyStateEvent(
            melody_name.clone(),
            MelodyStateEvent::AddNote(note_info),
        ));
    }
}
