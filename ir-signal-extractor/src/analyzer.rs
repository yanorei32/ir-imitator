use std::time::Duration;

use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator, VariantArray};

use model::{DataPoint, ValueChanges};

#[derive(VariantArray, EnumIter)]
pub enum CodeFormat {
    Aeha,
    Nec,
    Sony,
}

impl CodeFormat {
    pub const fn burst(&self) -> Duration {
        match self {
            Self::Aeha => Duration::from_micros(425 * 8),
            Self::Nec => Duration::from_micros(562 * 16),
            Self::Sony => Duration::from_micros(600 * 4),
        }
    }
}

#[derive(Debug)]
pub struct Analyzed<'a> {
    pub tracked_chunks: Vec<&'a [DataPoint]>,
}

pub fn analyze(values: &ValueChanges) -> Analyzed {
    let maximum_burst_time = CodeFormat::iter().map(|v| v.burst()).max().unwrap();

    let common_voltage = values.common_voltage();

    let blank_points: Vec<_> = values
        .datapoints
        .iter()
        .enumerate()
        .filter(|v| v.1.voltage == common_voltage)
        .filter(|v| v.1.delta_time > maximum_burst_time * 2)
        .map(|v| v.0)
        .collect();

    let tracked_chunks = blank_points
        .iter()
        .tuple_windows()
        .map(|(start, end)| &values.datapoints[(*start + 1)..*end])
        .collect();

    Analyzed { tracked_chunks }
}
