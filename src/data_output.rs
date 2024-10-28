use std::ops::Range;

use crate::data_output::MeasurementDataStatus::{
    ContaminationError, ContaminationWarning, Dazzle, NoReflection, Reflector, Valid,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Block {
    pub offset: u16,
    pub size: u16,
}

#[derive(Debug, PartialEq)]
pub struct DataOutputHeader {
    pub version: u8,
    pub version_major: u8,
    pub version_minor: u8,
    pub release: u8,
    pub device_serial_number: u32,
    pub system_plug_serial_number: u32,
    pub channel_number: u8,
    pub sequence_number: u32,
    pub scan_number: u32,
    pub time_stamp_date: u16,
    pub time_stamp_time: u32,
    pub device_status_block: Block,
    pub output_configuration_block: Block,
    pub measurement_data_block: Block,
    pub field_interruption_block: Block,
    pub application_data_block: Block,
    pub local_ios_block: Block,
}

impl DataOutputHeader {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            version: bytes[0],
            version_major: bytes[1],
            version_minor: bytes[2],
            release: bytes[3],
            device_serial_number: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            system_plug_serial_number: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            channel_number: bytes[12],
            sequence_number: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            scan_number: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
            time_stamp_date: u16::from_le_bytes(bytes[24..26].try_into().unwrap()),
            time_stamp_time: u32::from_le_bytes(bytes[28..32].try_into().unwrap()),
            device_status_block: Block {
                offset: u16::from_le_bytes(bytes[32..34].try_into().unwrap()),
                size: u16::from_le_bytes(bytes[34..36].try_into().unwrap()),
            },
            output_configuration_block: Block {
                offset: u16::from_le_bytes(bytes[36..38].try_into().unwrap()),
                size: u16::from_le_bytes(bytes[38..40].try_into().unwrap()),
            },
            measurement_data_block: Block {
                offset: u16::from_le_bytes(bytes[40..42].try_into().unwrap()),
                size: u16::from_le_bytes(bytes[42..44].try_into().unwrap()),
            },
            field_interruption_block: Block {
                offset: u16::from_le_bytes(bytes[44..46].try_into().unwrap()),
                size: u16::from_le_bytes(bytes[46..48].try_into().unwrap()),
            },
            application_data_block: Block {
                offset: u16::from_le_bytes(bytes[48..50].try_into().unwrap()),
                size: u16::from_le_bytes(bytes[50..52].try_into().unwrap()),
            },
            local_ios_block: Block {
                offset: u16::from_le_bytes(bytes[52..54].try_into().unwrap()),
                size: u16::from_le_bytes(bytes[54..56].try_into().unwrap()),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OutputConfigurationBlock {
    pub factor: u16,
    pub number_of_beams: u16,
    pub scan_cycle_time: u16,
    pub start_angle: i32,
    pub angular_resolution: f32,
    pub beam_interval: u32,
}

const START_ANGLE_DENOMINATOR: i32 = 4_194_304;
const ANGULAR_RESOLUTION_DENOMINATOR: f32 = 4_194_304.0;

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

#[derive(Debug, PartialEq)]
pub struct MeasurementDataBlock {
    pub number_of_beams: u32,
    pub measurements: Vec<Measurement>,
}

impl MeasurementDataBlock {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let number_of_beams = u32::from_le_bytes(bytes[0..4].try_into().unwrap());

        let mut measurements: Vec<Measurement> = Vec::new();
        measurements.reserve(number_of_beams as usize);

        for beam in 0..number_of_beams {
            let beam = beam as usize;
            let distance_bytes: Range<usize> = (4 + 4 * beam)..(4 + 4 * beam + 2);
            let rssi_bytes: usize = 4 + 4 * beam + 2;
            let status_bytes: usize = 4 + 4 * beam + 3;

            measurements.push(Measurement {
                distance: u16::from_le_bytes(bytes[distance_bytes].try_into().unwrap()),
                rssi: bytes[rssi_bytes],
                status: MeasurementDataStatus::from_byte(bytes[status_bytes]).unwrap(),
            });
        }

        Self {
            number_of_beams,
            measurements,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Measurement {
    pub distance: u16,
    pub rssi: u8,
    pub status: MeasurementDataStatus,
}

#[derive(Debug, PartialEq)]
pub enum MeasurementDataStatus {
    Valid,
    NoReflection,
    Dazzle,
    Reflector,
    ContaminationError,
    ContaminationWarning,
}

impl MeasurementDataStatus {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        const MASK: u8 = 0b0011_1110;
        let masked_byte = byte & MASK;
        return match masked_byte {
            0b0000_0000 => Ok(Valid),
            0b0000_0010 => Ok(NoReflection),
            0b0000_0100 => Ok(Dazzle),
            0b0000_1000 => Ok(Reflector),
            0b0001_0000 => Ok(ContaminationError),
            0b0010_0000 => Ok(ContaminationWarning),
            _ => Err(format!("unknown measurement data status: {}", byte).to_string()),
        };
    }

    pub fn to_byte(&self) -> u8 {
        return match self {
            Valid => 0b0000_0001,
            NoReflection => 0b0000_0010,
            Dazzle => 0b0000_0100,
            Reflector => 0b0000_1000,
            ContaminationError => 0b0001_0000,
            ContaminationWarning => 0b0010_0000,
        };
    }
}

#[cfg(test)]
mod data_output_header_tests {
    use array_concat::concat_arrays;

    use crate::data_output::{Block, DataOutputHeader};

    #[test]
    fn parse_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(result, expected_header);
    }

    fn create_valid_test_data() -> ([u8; 64], DataOutputHeader) {
        let version = [1];
        let version_major = [2];
        let version_minor = [3];
        let release = [4];
        let device_serial_number = [0x12, 0x34, 0x56, 0x78];
        let system_plug_serial_number = [0xAB, 0xCD, 0xEF, 0x01];
        let channel_number = [69];
        let reserved1 = [0; 3];
        let sequence_number = [0x34, 0x23, 0x12, 0x01];
        let scan_number = [0x69, 0x34, 0x37, 0xAD];
        let time_stamp_date = [0xAB, 0xBA];
        let reserved2 = [0; 2];
        let time_stamp_time = [0xEE, 0xFF, 0xC0, 0xDE];
        let block_device_status_offset = [0x01, 0x23];
        let block_device_status_size = [0x45, 0x67];
        let block_output_configuration_offset = [0x89, 0xAB];
        let block_output_configuration_size = [0xCD, 0xEF];
        let block_measurement_data_offset = [0xAE, 0x86];
        let block_measurement_data_size = [0x23, 0x32];
        let block_field_interruption_offset = [0x36, 0xA1];
        let block_field_interruption_size = [0x75, 0xA4];
        let block_application_data_offset = [0x90, 0x12];
        let block_application_data_size = [0x19, 0x96];
        let block_local_ios_offset = [0xAD, 0xFE];
        let block_local_ios_size = [0xCE, 0xBA];
        let padding = [0; 8];
        (
            concat_arrays!(
                version,
                version_major,
                version_minor,
                release,
                device_serial_number,
                system_plug_serial_number,
                channel_number,
                reserved1,
                sequence_number,
                scan_number,
                time_stamp_date,
                reserved2,
                time_stamp_time,
                block_device_status_offset,
                block_device_status_size,
                block_output_configuration_offset,
                block_output_configuration_size,
                block_measurement_data_offset,
                block_measurement_data_size,
                block_field_interruption_offset,
                block_field_interruption_size,
                block_application_data_offset,
                block_application_data_size,
                block_local_ios_offset,
                block_local_ios_size,
                padding
            ),
            DataOutputHeader {
                version: u8::from_le_bytes(version),
                version_major: u8::from_le_bytes(version_major),
                version_minor: u8::from_le_bytes(version_minor),
                release: u8::from_le_bytes(release),
                device_serial_number: u32::from_le_bytes(device_serial_number),
                system_plug_serial_number: u32::from_le_bytes(system_plug_serial_number),
                channel_number: u8::from_le_bytes(channel_number),
                sequence_number: u32::from_le_bytes(sequence_number),
                scan_number: u32::from_le_bytes(scan_number),
                time_stamp_date: u16::from_le_bytes(time_stamp_date),
                time_stamp_time: u32::from_le_bytes(time_stamp_time),
                device_status_block: Block {
                    offset: u16::from_le_bytes(block_device_status_offset),
                    size: u16::from_le_bytes(block_device_status_size),
                },
                output_configuration_block: Block {
                    offset: u16::from_le_bytes(block_output_configuration_offset),
                    size: u16::from_le_bytes(block_output_configuration_size),
                },
                measurement_data_block: Block {
                    offset: u16::from_le_bytes(block_measurement_data_offset),
                    size: u16::from_le_bytes(block_measurement_data_size),
                },
                field_interruption_block: Block {
                    offset: u16::from_le_bytes(block_field_interruption_offset),
                    size: u16::from_le_bytes(block_field_interruption_size),
                },
                application_data_block: Block {
                    offset: u16::from_le_bytes(block_application_data_offset),
                    size: u16::from_le_bytes(block_application_data_size),
                },
                local_ios_block: Block {
                    offset: u16::from_le_bytes(block_local_ios_offset),
                    size: u16::from_le_bytes(block_local_ios_size),
                },
            },
        )
    }
}

#[cfg(test)]
mod output_configuration_block_tests {
    use array_concat::concat_arrays;

    use crate::data_output::{
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

#[cfg(test)]
mod measurement_data_status_tests {
    const VALID_STATUS: u8 = 0b0000_0001;
    const NO_REFLECTION_STATUS: u8 = 0b0000_0010;
    const DAZZLE_STATUS: u8 = 0b0000_0100;
    const REFLECTOR_STATUS: u8 = 0b0000_1000;
    const CONTAMINATION_ERROR_STATUS: u8 = 0b0001_0000;
    const CONTAMINATION_WARNING_STATUS: u8 = 0b0010_0000;

    mod decoding {
        use crate::data_output::measurement_data_status_tests::{
            CONTAMINATION_ERROR_STATUS, CONTAMINATION_WARNING_STATUS, DAZZLE_STATUS,
            NO_REFLECTION_STATUS, REFLECTOR_STATUS, VALID_STATUS,
        };
        use crate::data_output::MeasurementDataStatus;

        #[test]
        fn decode_valid_status_from_byte() {
            let byte: u8 = VALID_STATUS;
            let status = MeasurementDataStatus::from_byte(byte).unwrap();
            assert_eq!(status, MeasurementDataStatus::Valid);
        }

        #[test]
        fn decode_no_reflection_status_from_byte() {
            let byte: u8 = NO_REFLECTION_STATUS;
            let status = MeasurementDataStatus::from_byte(byte).unwrap();
            assert_eq!(status, MeasurementDataStatus::NoReflection);
        }

        #[test]
        fn decode_dazzle_status_from_byte() {
            let byte: u8 = DAZZLE_STATUS;
            let status = MeasurementDataStatus::from_byte(byte).unwrap();
            assert_eq!(status, MeasurementDataStatus::Dazzle);
        }

        #[test]
        fn decode_reflector_status_from_byte() {
            let byte: u8 = REFLECTOR_STATUS;
            let status = MeasurementDataStatus::from_byte(byte).unwrap();
            assert_eq!(status, MeasurementDataStatus::Reflector);
        }

        #[test]
        fn decode_contamination_error_status_from_byte() {
            let byte: u8 = CONTAMINATION_ERROR_STATUS;
            let status = MeasurementDataStatus::from_byte(byte).unwrap();
            assert_eq!(status, MeasurementDataStatus::ContaminationError);
        }

        #[test]
        fn decode_contamination_warning_status_from_byte() {
            let byte: u8 = CONTAMINATION_WARNING_STATUS;
            let status = MeasurementDataStatus::from_byte(byte).unwrap();
            assert_eq!(status, MeasurementDataStatus::ContaminationWarning);
        }

        #[test]
        #[should_panic]
        fn decode_unknown_status_from_byte() {
            let byte: u8 = 0b1111_1111;
            let _ = MeasurementDataStatus::from_byte(byte).unwrap();
        }

        #[test]
        fn decode_valid_status_from_byte_ignoring_last_two_bits() {
            let byte: u8 = 0b1100_0001;
            let status = MeasurementDataStatus::from_byte(byte).unwrap();
            assert_eq!(status, MeasurementDataStatus::Valid);
        }

        #[test]
        fn decode_no_reflection_status_from_byte_with_valid_bit_set() {
            let byte: u8 = 0b0000_0011;
            let status = MeasurementDataStatus::from_byte(byte).unwrap();
            assert_eq!(status, MeasurementDataStatus::NoReflection);
        }
    }

    mod encoding {
        use crate::data_output::measurement_data_status_tests::{
            CONTAMINATION_ERROR_STATUS, CONTAMINATION_WARNING_STATUS, DAZZLE_STATUS,
            NO_REFLECTION_STATUS, REFLECTOR_STATUS, VALID_STATUS,
        };
        use crate::data_output::MeasurementDataStatus;

        #[test]
        fn encode_valid_status_to_byte() {
            let status = MeasurementDataStatus::Valid;
            let result = status.to_byte();
            assert_eq!(result, VALID_STATUS);
        }

        #[test]
        fn encode_no_reflection_status_to_byte() {
            let status = MeasurementDataStatus::NoReflection;
            let result = status.to_byte();
            assert_eq!(result, NO_REFLECTION_STATUS);
        }

        #[test]
        fn encode_dazzle_status_to_byte() {
            let status = MeasurementDataStatus::Dazzle;
            let result = status.to_byte();
            assert_eq!(result, DAZZLE_STATUS);
        }

        #[test]
        fn encode_reflector_status_to_byte() {
            let status = MeasurementDataStatus::Reflector;
            let result = status.to_byte();
            assert_eq!(result, REFLECTOR_STATUS);
        }

        #[test]
        fn encode_contamination_error_status_to_byte() {
            let status = MeasurementDataStatus::ContaminationError;
            let result = status.to_byte();
            assert_eq!(result, CONTAMINATION_ERROR_STATUS);
        }

        #[test]
        fn encode_contamination_warning_status_to_byte() {
            let status = MeasurementDataStatus::ContaminationWarning;
            let result = status.to_byte();
            assert_eq!(result, CONTAMINATION_WARNING_STATUS);
        }
    }
}

#[cfg(test)]
mod measurement_data_block_tests {
    use crate::data_output::{Measurement, MeasurementDataBlock, MeasurementDataStatus};

    #[test]
    fn parse_one_measurement_from_valid_block() {
        let number_of_beams = 1;
        let (test_data, expected_block) = create_valid_test_data(number_of_beams);
        let result = MeasurementDataBlock::from_bytes(&test_data);
        assert_eq!(result, expected_block);
    }

    #[test]
    fn parse_three_measurement_from_valid_block() {
        let number_of_beams = 3;
        let (test_data, expected_block) = create_valid_test_data(number_of_beams);
        let result = MeasurementDataBlock::from_bytes(&test_data);
        assert_eq!(result, expected_block);
    }

    fn create_valid_test_data(number_of_beams: u32) -> ([u8; 1024], MeasurementDataBlock) {
        let mut test_data: Vec<u8> = Vec::new();

        test_data.extend(number_of_beams.to_le_bytes());

        let mut measurements: Vec<Measurement> = Vec::new();

        for beam in 1..=number_of_beams {
            let distance = 100 * beam as u16;
            let rssi = 2 * beam as u8;
            let status = generate_random_status();

            test_data.extend(distance.to_le_bytes());
            test_data.extend(rssi.to_le_bytes());
            test_data.push(status.to_byte());

            measurements.push(Measurement {
                distance,
                rssi,
                status,
            });
        }

        if test_data.len() < 1024 {
            let padding: Vec<u8> = vec![0u8; 1024 - test_data.len()];
            test_data.extend(padding);
        }

        (
            test_data.try_into().unwrap(),
            MeasurementDataBlock {
                number_of_beams,
                measurements,
            },
        )
    }

    fn generate_random_status() -> MeasurementDataStatus {
        // TODO!
        MeasurementDataStatus::Valid
    }
}
