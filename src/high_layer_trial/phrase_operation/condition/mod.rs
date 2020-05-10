use super::super::super::data::music_info::{Beat, Note, Pitch};
use super::split_by_condition::Condition;

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
    pitch: Pitch,
}

impl PitchLarger {
    pub fn new(pitch: Pitch) -> Self {
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
