use std::sync::Arc;

use nom::branch::alt;
use nom::bytes::complete::take;
use nom::character::complete::char;
use nom::combinator::{iterator, not};
use nom::IResult;

use super::super::super::data::music_info::{Beat, Phrase, SampleNote};
use super::super::super::music_state::states::{MusicState, MusicStateEvent};
use super::super::super::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use super::super::super::players::player::Player;
use super::send_phrase::send_sample_phrase;

#[derive(Clone, Debug, PartialEq)]
enum Element {
    Sample(String),
    Tuplet(Tuplet),
}

#[derive(Clone, Debug, PartialEq)]
struct Tuplet {
    elements: Vec<Element>,
}

impl Tuplet {
    fn size(&self) -> usize {
        self.elements.len()
    }
}

fn parse_elements(s: &str) -> IResult<&str, Vec<Element>> {
    let mut it = iterator(s, parse_element);
    let elements = it.collect();
    let (s, _) = it.finish()?;

    Ok((s, elements))
}

fn parse_element(s: &str) -> IResult<&str, Element> {
    let (s, _) = not(char(']'))(s)?;
    alt((parse_tuplet, parse_sample))(s)
}

fn parse_tuplet(s: &str) -> IResult<&str, Element> {
    let (s, _) = char('[')(s)?;

    let mut it = iterator(s, parse_element);
    let elements = it.collect();
    let (s, _) = it.finish()?;
    let (s, _) = char(']')(s)?;

    Ok((s, Element::Tuplet(Tuplet { elements })))
}

fn parse_sample(s: &str) -> IResult<&str, Element> {
    let (s, sample_str) = take(1u8)(s)?;
    Ok((s, Element::Sample(sample_str.to_string())))
}

fn tuplet_to_notes(tuplet: Tuplet, start: Beat, duration: Beat) -> Vec<SampleNote> {
    let length_unit: Beat = duration / tuplet.size() as f32;
    let mut now = start;
    let mut ret_notes = vec![];
    for element in tuplet.elements.iter() {
        match element {
            Element::Sample(sound) => {
                if sound != " " {
                    let note = SampleNote {
                        sound: sound.to_string(),
                        start: now,
                    };
                    ret_notes.push(note)
                }
            }
            Element::Tuplet(tuplet) => {
                let notes = tuplet_to_notes(tuplet.clone(), now, length_unit);
                for note in notes.iter() {
                    ret_notes.push(note.clone());
                }
            }
        }
        now = now + length_unit;
    }
    ret_notes
}

pub fn parse_sample_lang(s: String) -> Phrase<SampleNote> {
    let mut now: Beat = Beat::from(0);
    let length_unit: Beat = Beat::from(0.5);
    let mut phrase = Phrase::new();

    let (_, elements) = parse_elements(&s).unwrap();
    phrase = phrase.set_length(Beat::from(elements.len() as f32 / 2.0));

    for element in elements.iter() {
        match element {
            Element::Sample(sound) => {
                if sound != " " {
                    let note = SampleNote {
                        sound: sound.to_string(),
                        start: now,
                    };
                    phrase = phrase.add_note(note);
                }
            }
            Element::Tuplet(tuplet) => {
                let notes = tuplet_to_notes(tuplet.clone(), now, length_unit);
                for note in notes.iter() {
                    phrase = phrase.add_note(note.clone());
                }
            }
        }
        now = now + length_unit;
    }

    phrase
}

pub fn send_sample_lang(
    phrase_string: String,
    section_beat: Beat,
    phrase_name: String,
    sample_name: String,
    vol: f32,
    pan: f32,
    player: Arc<
        dyn Player<MusicState, MusicStateEvent, WaveReader, (Vec<i16>, Vec<i16>), WaveReaderEvent>,
    >,
) -> Result<(), String> {
    send_sample_phrase(
        parse_sample_lang(phrase_string),
        section_beat,
        phrase_name,
        sample_name,
        vol,
        pan,
        player,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_elements() {
        let s = "xo[-x]";
        let true_elements = vec![
            Element::Sample("x".to_string()),
            Element::Sample("o".to_string()),
            Element::Tuplet(Tuplet {
                elements: vec![
                    Element::Sample("-".to_string()),
                    Element::Sample("x".to_string()),
                ],
            }),
        ];

        let (_, elements) = parse_elements(s).unwrap();

        assert_eq!(elements, true_elements);
    }
}
