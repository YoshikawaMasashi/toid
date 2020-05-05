use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::super::data::music_info::beat::Beat;
use super::super::data::music_info::note::Note;
use super::super::data::music_info::phrase::Phrase;
use super::super::state_management::serialize;
use super::super::state_management::state::State;

#[derive(Serialize, Deserialize)]
pub struct PhraseState {
    pub phrase: Phrase,
}

impl PhraseState {
    pub fn add_note(&self, note: Note) -> Self {
        PhraseState {
            phrase: self.phrase.add_note(note),
        }
    }

    pub fn set_repeat_length(&self, repeat_length: Beat) -> Self {
        PhraseState {
            phrase: self.phrase.set_repeat_length(repeat_length),
        }
    }
}

impl State<PhraseStateEvent> for PhraseState {
    fn new() -> Self {
        let phrase = Phrase {
            notes: BTreeMap::new(),
            repeat_length: Beat::from(8),
        };
        PhraseState { phrase }
    }

    fn reduce(&self, event: PhraseStateEvent) -> Self {
        match event {
            PhraseStateEvent::AddNote(note) => self.add_note(note),
        }
    }
}

impl serialize::Serialize<PhraseState> for PhraseState {
    fn serialize(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(err) => Err(format!("error in serizalization : {}", err)),
        }
    }
    fn deserialize(serialized: String) -> Result<Self, String> {
        match serde_json::from_str(serialized.as_str()) {
            Ok(deserialized) => Ok(deserialized),
            Err(err) => Err(format!("error in deserizalization : {}", err)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum PhraseStateEvent {
    AddNote(Note),
}

impl serialize::Serialize<PhraseStateEvent> for PhraseStateEvent {
    fn serialize(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(err) => Err(format!("error in serizalization : {}", err)),
        }
    }
    fn deserialize(serialized: String) -> Result<Self, String> {
        match serde_json::from_str(serialized.as_str()) {
            Ok(deserialized) => Ok(deserialized),
            Err(err) => Err(format!("error in deserizalization : {}", err)),
        }
    }
}
