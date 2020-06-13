use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound::{Excluded, Included, Unbounded};

use itertools::izip;

use super::super::super::data::music_info::{Beat, Phrase, Pitch, PitchNote, Scale};

pub fn round_line(
    line_beat: Vec<Beat>,
    line_pitch: Vec<Pitch>,
    start: Vec<Beat>,
    duration: Vec<Beat>,
    scale: Scale,
) -> Phrase<PitchNote> {
    let mut phrase = Phrase::new();

    let mut line_map: BTreeMap<Beat, Pitch> = BTreeMap::new();
    for (&b, &p) in line_beat.iter().zip(line_pitch.iter()) {
        line_map.insert(b, p);
    }
    let line_map = line_map;

    let mut scale_set: BTreeSet<Pitch> = BTreeSet::new();
    for &p in scale.to_pitch_in_octave_vec().iter() {
        for i in 0..10 {
            scale_set.insert(Pitch::from(p.pitch + 12.0 * i as f32));
        }
    }
    let scale_set = scale_set;

    let mut not_round_pitch: Vec<Pitch> = Vec::new();
    for &start_beat in start.iter() {
        let left_map = line_map.range((Excluded(&start_beat), Unbounded)).next();
        let right_map = line_map
            .range((Unbounded, Included(&start_beat)))
            .rev()
            .next();
        let not_round_pitch_ = match (left_map, right_map) {
            (Some((left_beat, &left_pitch)), Some((right_beat, &right_pitch))) => Pitch::from(
                (start_beat.to_f32() - left_beat.to_f32())
                    / (right_beat.to_f32() - left_beat.to_f32())
                    * left_pitch.to_f32()
                    + (right_beat.to_f32() - start_beat.to_f32())
                        / (right_beat.to_f32() - left_beat.to_f32())
                        * right_pitch.to_f32(),
            ),
            (Some((_, &left_pitch)), None) => left_pitch,
            (None, Some((_, &right_pitch))) => right_pitch,
            (None, None) => {
                return phrase;
            }
        };
        not_round_pitch.push(not_round_pitch_);
    }

    for (&start_beat, &not_round_pitch_, &duration_beat) in
        izip!(start.iter(), not_round_pitch.iter(), duration.iter())
    {
        let up_pitch = scale_set
            .range((Included(&not_round_pitch_), Unbounded))
            .next();
        let down_pitch = scale_set
            .range((Unbounded, Included(&not_round_pitch_)))
            .next();
        match (up_pitch, down_pitch) {
            (Some(&up_pitch), Some(&down_pitch)) => {
                if (up_pitch - not_round_pitch_).abs() > (down_pitch - not_round_pitch_).abs() {
                    phrase = phrase.add_note(PitchNote {
                        pitch: down_pitch,
                        start: start_beat,
                        duration: duration_beat,
                    });
                } else {
                    phrase = phrase.add_note(PitchNote {
                        pitch: up_pitch,
                        start: start_beat,
                        duration: duration_beat,
                    });
                }
            }
            (Some(&up_pitch), None) => {
                phrase = phrase.add_note(PitchNote {
                    pitch: up_pitch,
                    start: start_beat,
                    duration: duration_beat,
                });
            }
            (None, Some(&down_pitch)) => {
                phrase = phrase.add_note(PitchNote {
                    pitch: down_pitch,
                    start: start_beat,
                    duration: duration_beat,
                });
            }
            (None, None) => {
                return phrase;
            }
        }
    }
    phrase = phrase.set_length(start[start.len() - 1] + duration[duration.len() - 1]);

    phrase
}
