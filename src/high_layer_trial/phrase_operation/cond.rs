use super::super::super::data::music_info::{Beat, Note, Phrase};

pub trait Condition {
    fn judge(&self, note: Note) -> bool;
}

pub struct And {
    condition1: Box<dyn Condition>,
    condition2: Box<dyn Condition>,
}

impl And {
    pub fn new(condition1: Box<dyn Condition>, condition2: Box<dyn Condition>) -> Self {
        Self {
            condition1,
            condition2,
        }
    }
}

impl Condition for And {
    fn judge(&self, note: Note) -> bool {
        self.condition1.judge(note) && self.condition2.judge(note)
    }
}

pub struct Or {
    condition1: Box<dyn Condition>,
    condition2: Box<dyn Condition>,
}

impl Or {
    pub fn new(condition1: Box<dyn Condition>, condition2: Box<dyn Condition>) -> Self {
        Self {
            condition1,
            condition2,
        }
    }
}

impl Condition for Or {
    fn judge(&self, note: Note) -> bool {
        self.condition1.judge(note) || self.condition2.judge(note)
    }
}

pub struct Not {
    condition: Box<dyn Condition>,
}

impl Not {
    pub fn new(condition: Box<dyn Condition>) -> Self {
        Self { condition }
    }
}

impl Condition for Not {
    fn judge(&self, note: Note) -> bool {
        !self.condition.judge(note)
    }
}

pub struct PitchLarger {
    pitch: f32,
}

impl PitchLarger {
    pub fn new(pitch: f32) -> Self {
        Self { pitch }
    }
}

impl Condition for PitchLarger {
    fn judge(&self, note: Note) -> bool {
        note.pitch > self.pitch
    }
}

pub struct StartLarger {
    start: Beat,
}

impl StartLarger {
    pub fn new(start: Beat) -> Self {
        Self { start }
    }
}

impl Condition for StartLarger {
    fn judge(&self, note: Note) -> bool {
        note.start > self.start
    }
}

pub struct IsDownBeat {}

impl IsDownBeat {
    pub fn new() -> Self {
        Self {}
    }
}

impl Condition for IsDownBeat {
    fn judge(&self, note: Note) -> bool {
        let start_in_beat = note.start % Beat::from(1);
        start_in_beat >= Beat::from(0.75) || start_in_beat < Beat::from(0.25)
    }
}

pub fn split_by_condition(phrase: Phrase, condition: Box<dyn Condition>) -> (Phrase, Phrase) {
    let mut true_phrase = Phrase::new();
    let mut false_phrase = Phrase::new();
    true_phrase = true_phrase.set_length(phrase.length);
    false_phrase = false_phrase.set_length(phrase.length);

    for (_, note_vec) in phrase.notes.iter() {
        for &note in note_vec.iter() {
            if condition.judge(note) {
                true_phrase = true_phrase.add_note(note.clone());
            } else {
                false_phrase = false_phrase.add_note(note.clone());
            }
        }
    }

    (true_phrase, false_phrase)
}
