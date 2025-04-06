use std::io::Read;

use csv::Reader;

use model::{DataPoint, ValueChanges, Voltage};

pub fn parse<R: Read>(reader: R) -> ValueChanges {
    let mut reader = Reader::from_reader(reader);

    let mut datapoints = vec![];
    let mut records = reader.records();

    let first_record = records.next().unwrap().unwrap();

    let mut previous_time = iso8601_timestamp::Timestamp::parse(first_record.get(0).unwrap()).unwrap();
    let mut previous_voltage: Voltage = first_record.get(1).unwrap().parse().unwrap();

    for record in records {
        let record = record.unwrap();

        let time = iso8601_timestamp::Timestamp::parse(record.get(0).unwrap()).unwrap();
        let voltage: Voltage = record.get(1).unwrap().parse().unwrap();

        let delta_time: std::time::Duration = time.duration_since(previous_time).try_into().unwrap();

        datapoints.push(DataPoint {
            delta_time,
            voltage: previous_voltage,
        });

        previous_time = time;
        previous_voltage = voltage;
    }

    ValueChanges { datapoints }
}
