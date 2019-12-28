use std::sync::Arc;

use super::generator::Generator;

enum SampleType {
    Monoral,
    Right,
    Left,
    LinkSample,
}

pub struct Sample {
    sample_access: Arc<Vec<i16>>,
    name: String,
    start: u16,
    end: u16,
    loopstart: u16,
    loopend: u16,
    sample_rate: u16,
    original_key: u8,
    correction: i8,
    sample_link: Option<Arc<Sample>>,
    typee: SampleType,
}
