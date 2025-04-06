use std::ops::Not;
use std::str::FromStr;
use std::time::Duration;

use linearize::{Linearize, StaticMap};
use serde::{Serialize, Deserialize};
use strum::Display;

#[derive(Debug, Clone, Copy, Linearize, Display, Eq, PartialEq)]
pub enum Voltage {
    Low,
    High,
}

impl FromStr for Voltage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::Low,
            "L" => Self::Low,
            "1" => Self::High,
            "H" => Self::High,
            _ => return Err(()),
        })
    }
}

impl Not for Voltage {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
}

#[derive(Debug)]
pub struct DataPoint {
    pub voltage: Voltage,
    pub delta_time: std::time::Duration,
}

#[derive(Debug)]
pub struct ValueChanges {
    pub datapoints: Vec<DataPoint>,
}

impl ValueChanges {
    pub fn common_voltage(&self) -> Voltage {
        let mut durations: StaticMap<Voltage, Duration> = StaticMap::default();

        for datapoint in &self.datapoints {
            durations[datapoint.voltage] += datapoint.delta_time;
        }

        durations.iter().max_by_key(|d| d.1).unwrap().0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub frequency_khz: u16,
    pub datapoints: Vec<u16>,
}

impl Packet {
    pub fn encode(&self) -> Vec<u8> {
        // frequency
        let mut data = vec![self.frequency_khz];

        // data-points
        data.extend_from_slice(&self.datapoints);

        // check-sum
        data.push(
            data.iter()
                .fold(0u16, |cum, value| cum.wrapping_add(*value)),
        );

        // as u8
        data.iter().flat_map(|v| v.to_le_bytes()).collect()
    }
}
