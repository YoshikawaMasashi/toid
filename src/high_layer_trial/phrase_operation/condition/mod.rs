use super::super::super::data::music_info::{Beat, Phrase, Pitch};

pub fn and(cond1: Vec<bool>, cond2: Vec<bool>) -> Vec<bool> {
    let mut new_cond = vec![];
    for (&c1, &c2) in cond1.iter().zip(cond2.iter()) {
        new_cond.push(c1 && c2);
    }
    new_cond
}

pub fn or(cond1: Vec<bool>, cond2: Vec<bool>) -> Vec<bool> {
    let mut new_cond = vec![];
    for (&c1, &c2) in cond1.iter().zip(cond2.iter()) {
        new_cond.push(c1 || c2);
    }
    new_cond
}

pub fn not(cond: Vec<bool>) -> Vec<bool> {
    let mut new_cond = vec![];
    for &c in cond.iter() {
        new_cond.push(!c);
    }
    new_cond
}

pub fn pitch_larger(phrase: Phrase, pitch: Pitch) -> Vec<bool> {
    let mut new_cond = vec![];
    for &note in phrase.note_vec().iter() {
        new_cond.push(note.pitch > pitch);
    }
    new_cond
}

pub fn start_larger(phrase: Phrase, start: Beat) -> Vec<bool> {
    let mut new_cond = vec![];
    for &note in phrase.note_vec().iter() {
        new_cond.push(note.start > start);
    }
    new_cond
}

pub fn is_down_beat(phrase: Phrase) -> Vec<bool> {
    let mut new_cond = vec![];
    for &note in phrase.note_vec().iter() {
        let start_in_beat = note.start % Beat::from(1);
        new_cond.push(start_in_beat >= Beat::from(0.75) || start_in_beat < Beat::from(0.25));
    }
    new_cond
}
