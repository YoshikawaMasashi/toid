use std::fmt;
use std::sync::Arc;

use super::super::riff::{RiffChank, RiffData};

pub struct Wave {}

impl Wave {
    pub fn parse(i: &[u8]) -> Result<Self, String> {
        let chank = RiffChank::parse(i)?;
        Ok(Wave {})
    }
}
