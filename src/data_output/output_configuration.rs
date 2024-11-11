use std::convert::TryInto;

const START_ANGLE_DENOMINATOR: i32 = 4_194_304;
const ANGULAR_RESOLUTION_DENOMINATOR: f32 = 4_194_304.0;

#[derive(Debug, PartialEq)]
pub struct OutputConfigurationBlock {
    pub factor: u16,
    pub number_of_beams: u16,
    pub scan_cycle_time: u16,
    pub start_angle: i32,
    pub angular_resolution: f32,
    pub beam_interval: u32,
}

impl OutputConfigurationBlock {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            factor: u16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            number_of_beams: u16::from_le_bytes(bytes[2..4].try_into().unwrap()),
            scan_cycle_time: u16::from_le_bytes(bytes[4..6].try_into().unwrap()),
            start_angle: i32::from_le_bytes(bytes[8..12].try_into().unwrap())
                / START_ANGLE_DENOMINATOR,
            angular_resolution: (i32::from_le_bytes(bytes[12..16].try_into().unwrap()) as f32)
                / ANGULAR_RESOLUTION_DENOMINATOR,
            beam_interval: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
        }
    }
}

#[cfg(test)]
mod output_configuration_block_tests {
    use array_concat::concat_arrays;

    use crate::data_output::output_configuration::{
        OutputConfigurationBlock, ANGULAR_RESOLUTION_DENOMINATOR, START_ANGLE_DENOMINATOR,
    };

    #[test]
    fn parse_from_valid_block() {
        let (test_data, expected_block) = create_valid_test_data();
        let result = OutputConfigurationBlock::from_bytes(&test_data);
        assert_eq!(result, expected_block);
    }

    fn create_valid_test_data() -> ([u8; 24], OutputConfigurationBlock) {
        let factor = [0x12, 0x23];
        let number_of_beams = [0x86, 0xAE];
        let scan_cycle_time = [0x45, 0x67];
        let reserved1 = [0; 2];
        let start_angle = [0xC0, 0xFF, 0xEE, 0x69];
        let angular_resolution = [0xAB, 0xBA, 0xFE, 0xC0];
        let beam_interval = [0x01, 0x23, 0x45, 0x56];
        let reserved2 = [0; 4];
        (
            concat_arrays!(
                factor,
                number_of_beams,
                scan_cycle_time,
                reserved1,
                start_angle,
                angular_resolution,
                beam_interval,
                reserved2
            ),
            OutputConfigurationBlock {
                factor: u16::from_le_bytes(factor),
                number_of_beams: u16::from_le_bytes(number_of_beams),
                scan_cycle_time: u16::from_le_bytes(scan_cycle_time),
                start_angle: i32::from_le_bytes(start_angle) / START_ANGLE_DENOMINATOR,
                angular_resolution: (i32::from_le_bytes(angular_resolution) as f32)
                    / ANGULAR_RESOLUTION_DENOMINATOR,
                beam_interval: u32::from_le_bytes(beam_interval),
            },
        )
    }
}
