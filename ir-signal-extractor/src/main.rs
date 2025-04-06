use clap::Parser;
use std::fs::File;
use std::path::PathBuf;

mod analyzer;
mod saleae_csv;

#[derive(Debug, Clone, Parser)]
struct Cli {
    csv_paths: Vec<PathBuf>,
}

pub fn printf_encode(data: &[u8]) -> String {
    let mut buffer = String::new();

    buffer.push_str("printf '");

    for datum in data {
        buffer.push_str(&format!("\\x{:02x}", datum));
    }

    buffer.push_str("' > /dev/udp/$HOST/6464");
    buffer
}

fn main() {
    let cli = Cli::parse();

    for csv_path in cli.csv_paths {
        let file = File::open(&csv_path).unwrap();
        let parsed = saleae_csv::parse(file);
        let analyzed = analyzer::analyze(&parsed);

        for (n, chunk) in analyzed.tracked_chunks.iter().enumerate() {
            let packet = model::Packet {
                frequency_khz: 38,
                datapoints: chunk
                    .iter()
                    .map(|d| d.delta_time.as_micros() as u16)
                    .collect(),
            };

            let mut json_path = csv_path.clone();

            json_path.set_file_name(format!(
                "{}.{n}.json",
                csv_path.file_name().unwrap().to_str().unwrap()
            ));

            let json_file = File::create(json_path).unwrap();
            serde_json::to_writer_pretty(json_file, &packet).unwrap();

            println!("---- DETECTED {n} ----");
            println!("{}", printf_encode(&packet.encode()));

            println!();
        }
    }
}
