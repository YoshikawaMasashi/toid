use rand::seq::SliceRandom;
use rand::thread_rng;

use super::super::super::data::music_info::{Beat, Note, Phrase};

pub fn shuffle_start(phrase: Phrase) -> Phrase {
    let mut new_phrase = Phrase::new();
    new_phrase = new_phrase.set_length(phrase.length);

    let mut new_starts: Box<[Beat]> = phrase.notes.keys().cloned().collect();
    let mut rng = thread_rng();
    new_starts.shuffle(&mut rng);
    for ((_, note_vec), &new_start) in phrase.notes.iter().zip(new_starts.iter()) {
        for note in note_vec.iter() {
            new_phrase = new_phrase.add_note(Note {
                pitch: note.pitch,
                duration: note.duration,
                start: new_start,
            });
        }
    }

    new_phrase
}
