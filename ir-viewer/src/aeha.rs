pub const T_US: u16 = 425;
pub const LEADER_T_COUNT: u16 = 8;
pub const REPEAT_BLANK_T_COUNT: u16 = 8;
pub const FRAME_BLANK_T_COUNT: u16 = 4;
pub const ZERO_BLANK_T_COUNT: u16 = 1;
pub const ONE_BLANK_T_COUNT: u16 = 3;

#[derive(Debug, Clone)]
pub enum Signal {
    Repeat,
    Frame(Vec<bool>),
}

fn decode_bits(timings: &[u16]) -> Vec<bool> {
    timings
        .iter()
        .enumerate()
        .filter(|(n, _)| n % 2 == 1)
        .map(|(_, blank_us)| {
            let standard_zero_blank_us = T_US * ZERO_BLANK_T_COUNT;
            let zero_differencial =
                ((*blank_us as f32 / standard_zero_blank_us as f32) - 1.0).abs();

            let standard_one_blank_us = T_US * ONE_BLANK_T_COUNT;
            let one_differencial = ((*blank_us as f32 / standard_one_blank_us as f32) - 1.0).abs();

            one_differencial < zero_differencial
        })
        .collect()
}

impl Signal {
    pub fn from_timings(timings: &[u16]) -> Self {
        let blank_us = timings[1];

        let standard_repeat_blank_us = T_US * REPEAT_BLANK_T_COUNT;
        let repeat_differencial = ((blank_us as f32 / standard_repeat_blank_us as f32) - 1.0).abs();

        let standard_frame_blank_us = T_US * FRAME_BLANK_T_COUNT;
        let frame_differencial = ((blank_us as f32 / standard_frame_blank_us as f32) - 1.0).abs();

        if repeat_differencial < frame_differencial {
            return Self::Repeat;
        }

        let bits = decode_bits(&timings[2..]);

        return Self::Frame(bits);
    }
}
