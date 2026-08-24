use clap::Parser;
use model::Packet;
use ordered_float::OrderedFloat;
use std::path::PathBuf;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tracing_subscriber::prelude::*;

mod aeha;

#[derive(Clone, Debug, Parser)]
struct Cli {
    timings: PathBuf,
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq)]
enum Formats {
    NEC,
    AEHA,
    SONY,
}

impl Formats {
    fn t_us(&self) -> u16 {
        match self {
            Self::NEC => 562,
            Self::AEHA => aeha::T_US,
            Self::SONY => 600,
        }
    }

    fn leader_t_count(&self) -> u16 {
        match self {
            Self::NEC => 16,
            Self::AEHA => aeha::LEADER_T_COUNT,
            Self::SONY => 4,
        }
    }

    fn estimate_format_from_leader_us(leader_us: u16) -> Self {
        Self::iter()
            .min_by_key(|format| {
                let standard_leader_us = format.t_us() * format.leader_t_count();
                let differencial = ((leader_us as f32 / standard_leader_us as f32) - 1.0).abs();
                tracing::debug!(
                    "Format: {format:?}, Differencial: {}%",
                    differencial * 100.0
                );
                OrderedFloat::from(differencial)
            })
            .unwrap()
    }
}

fn format_bits(bits: &[bool]) -> String {
    let mut s = String::new();
    for bit in bits {
        if *bit {
            s.push('1');
        } else {
            s.push('0');
        }
    }
    s
}

fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cli = Cli::parse();
    let timings = std::fs::read_to_string(&cli.timings).expect("The timings file can be open");
    let timings: Packet = serde_json::from_str(&timings).expect("The timings can be parse");

    let leader_us = timings.datapoints.first().unwrap();
    let format = Formats::estimate_format_from_leader_us(*leader_us);

    match format {
        Formats::AEHA => {
            let signal = aeha::Signal::from_timings(&timings.datapoints);
            match signal {
                aeha::Signal::Repeat => println!("AEHA Repeat"),
                aeha::Signal::Frame(bits) => println!("AEHA bits: {}", format_bits(&bits)),
            }
        },
        format => unimplemented!("{format:?}"),
    }
}
