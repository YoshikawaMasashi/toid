use std::sync::Arc;

use nom::branch::alt;
use nom::bytes::complete::take;
use nom::character::complete::{char, one_of, digit1};
use nom::combinator::{iterator, not};
use nom::IResult;

use super::super::data::music_info::{Beat, Phrase, PitchNote, PitchInterval, Pitch};
use super::super::music_state::states::{MusicState, MusicStateEvent};
use super::super::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use super::super::players::player::Player;
use super::music_language::send_phrase::send_sample_phrase;
use super::phrase_operation::change_key;

fn parse_phrase(s: &str) -> IResult<&str, Phrase<PitchNote>> {
    let mut it = iterator(s, one_of("+-"));
    let pm_chrs: Vec<char> = it.collect();
    let (s, _) = it.finish()?;
    let mut octave: i32 = 0;
    for pm_chr in pm_chrs.iter() {
        match pm_chr {
            '+' => {
                octave += 1;
            },
            '-' => {
                octave -= 1;
            },
            _ => {}
        }
    }

    let (s, elements) = parse_elements(s)?;

    let mut phrase = Phrase::new();
    phrase = phrase.set_length(Beat::from(elements.len() as f32 / 2.0));

    let mut now: Beat = Beat::from(0);
    let length_unit: Beat = Beat::from(0.5);

    for element in elements.iter() {
        match element {
            Element::Pitch(pitch_element) => {
                let pitch = match pitch_element.pitch % 7 {
                    0 => 47.0,
                    1 => 48.0,
                    2 => 50.0,
                    3 => 52.0,
                    4 => 53.0,
                    5 => 55.0,
                    6 => 57.0,
                    _ => unreachable!(),
                };
                let pitch = pitch + (pitch_element.pitch / 7) as f32 * 12.0;
                let pitch = pitch + pitch_element.shift as f32;

                let note = PitchNote {
                    pitch: Pitch::from(pitch),
                    start: now,
                    duration: length_unit,
                };
                phrase = phrase.add_note(note);
            }
            Element::Tuplet(tuplet_element) => {
                let notes = tuplet_to_notes(tuplet_element.clone(), now, length_unit);
                for note in notes.iter() {
                    phrase = phrase.add_note(note.clone());
                }
            }
            Element::Rest() => {
            }
        }
        now = now + length_unit;
    }

    phrase = change_key(phrase, PitchInterval::from(12.0 * octave as f32));

    Ok((s, phrase))
}


fn tuplet_to_notes(tuplet_element: TupletElement, start: Beat, duration: Beat) -> Vec<PitchNote> {
    let length_unit: Beat = duration / tuplet_element.size() as f32;
    let mut now = start;
    let mut ret_notes = vec![];
    // TODO
    for element in tuplet_element.elements.iter() {
        match element {
            Element::Pitch(pitch_element) => {
                let pitch = match pitch_element.pitch % 7 {
                    0 => 47.0,
                    1 => 48.0,
                    2 => 50.0,
                    3 => 52.0,
                    4 => 53.0,
                    5 => 55.0,
                    6 => 57.0,
                    _ => unreachable!(),
                };
                let pitch = pitch + (pitch_element.pitch / 7) as f32 * 12.0;
                let pitch = pitch + pitch_element.shift as f32;

                let note = PitchNote {
                    pitch: Pitch::from(pitch),
                    start: now,
                    duration: length_unit,
                };
                ret_notes.push(note);
            }
            Element::Rest() => {

            }
            Element::Tuplet(tuplet_element) => {
                let notes = tuplet_to_notes(tuplet_element.clone(), now, length_unit);
                for note in notes.iter() {
                    ret_notes.push(note.clone());
                }
            }
        }
        now = now + length_unit;
    }
    ret_notes
}

#[derive(Clone, Debug, PartialEq)]
enum Element {
    Pitch(PitchElement),
    Tuplet(TupletElement),
    Rest(),
}

#[derive(Clone, Debug, PartialEq)]
struct PitchElement {
    pitch: i32,
    shift: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct TupletElement {
    elements: Vec<Element>,
}

impl TupletElement {
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
    alt((parse_tuplet, parse_pitch, parse_bracket_pitch, parse_rest))(s)
}

fn parse_tuplet(s: &str) -> IResult<&str, Element> {
    let (s, _) = char('[')(s)?;

    let mut it = iterator(s, parse_element);
    let elements = it.collect();
    let (s, _) = it.finish()?;
    let (s, _) = char(']')(s)?;

    Ok((s, Element::Tuplet(TupletElement { elements })))
}

fn parse_pitch(s: &str) -> IResult<&str, Element> {
    let (s, pitch_chr) = one_of("0123456789")(s)?;
    let mut it = iterator(s, one_of("+-"));
    let pm_chrs: Vec<char> = it.collect();
    let (s, _) = it.finish()?;
    let mut shift: i32 = 0;
    for pm_chr in pm_chrs.iter() {
        match pm_chr {
            '+' => {
                shift += 1;
            },
            '-' => {
                shift -= 1;
            },
            _ => {}
        }
    }

    let pitch: i32 = pitch_chr as i32 - 48; // char to num
    Ok((s, Element::Pitch(
        PitchElement{
            pitch,
            shift
        }
    )))
}

fn parse_bracket_pitch(s: &str) -> IResult<&str, Element> {
    let (s, _) = char('(')(s)?;

    let mut it = iterator(s, one_of("+-"));
    let pm_chrs: Vec<char> = it.collect();
    let (s, _) = it.finish()?;
    let mut octave: i32 = 0;
    for pm_chr in pm_chrs.iter() {
        match pm_chr {
            '+' => {
                octave += 1;
            },
            '-' => {
                octave -= 1;
            },
            _ => {}
        }
    }

    let (s, pitch_str) = digit1(s)?;
    let pitch: i32 = pitch_str.parse().unwrap();


    let mut it = iterator(s, one_of("+-"));
    let pm_chrs: Vec<char> = it.collect();
    let (s, _) = it.finish()?;
    let mut shift: i32 = 0;
    for pm_chr in pm_chrs.iter() {
        match pm_chr {
            '+' => {
                shift += 1;
            },
            '-' => {
                shift -= 1;
            },
            _ => {}
        }
    }

    let (s, _) = char(')')(s)?;

    let pitch: i32 = pitch + 7 * octave;
    Ok((s, Element::Pitch(
        PitchElement{
            pitch,
            shift
        }
    )))
}

fn parse_rest(s: &str) -> IResult<&str, Element> {
    let (s, _) = char(' ')(s)?;
    Ok((s, Element::Rest()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_elements() {
        let s = "13[2+1(+2-) ](13)";
        let true_elements = vec![
            Element::Pitch(PitchElement{pitch:1, shift:0}),
            Element::Pitch(PitchElement{pitch:3, shift:0}),
            Element::Tuplet(TupletElement {
                elements: vec![
                    Element::Pitch(PitchElement{pitch:2, shift:1}),
                    Element::Pitch(PitchElement{pitch:1, shift:0}),
                    Element::Pitch(PitchElement{pitch:9, shift:-1}),
                    Element::Rest(),
                ],
            }),
            Element::Pitch(PitchElement{pitch:13, shift:0}),
        ];

        let (_, elements) = parse_elements(s).unwrap();

        assert_eq!(elements, true_elements);
    }
}

