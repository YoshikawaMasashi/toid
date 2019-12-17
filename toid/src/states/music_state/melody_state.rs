use std::collections::BTreeMap;

enum CurrentMelodyState {
    On(f32, i64),
    Off,
}

enum MelodyEvent {
    On(f32),
    Off,
}

pub struct MelodyState {
    event_seq: BTreeMap<f32, MelodyEvent>,
    current_melody: CurrentMelodyState,
}
