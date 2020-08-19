use std::sync::Arc;

use nom::branch::alt;
use nom::bytes::complete::take;
use nom::character::complete::{char, one_of, digit1};
use nom::combinator::{iterator, not};
use nom::IResult;

use super::super::data::music_info::{Beat, Phrase, SampleNote};
use super::super::music_state::states::{MusicState, MusicStateEvent};
use super::super::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use super::super::players::player::Player;
use super::music_language::send_phrase::send_sample_phrase;

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

