use crate::data_output::measurement_data::MeasurementDataStatus::{
    ContaminationError, ContaminationWarning, Dazzle, NoReflection, Reflector, Valid,
};
use std::convert::TryInto;
use std::ops::Range;

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

// TODO: checkout out how is it implemented in the C++ driver
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
mod measurement_data_status_tests {
    const VALID_STATUS: u8 = 0b0000_0001;
    const NO_REFLECTION_STATUS: u8 = 0b0000_0010;
    const DAZZLE_STATUS: u8 = 0b0000_0100;
    const REFLECTOR_STATUS: u8 = 0b0000_1000;
    const CONTAMINATION_ERROR_STATUS: u8 = 0b0001_0000;
    const CONTAMINATION_WARNING_STATUS: u8 = 0b0010_0000;

    mod decoding {
        use crate::data_output::measurement_data::measurement_data_status_tests::{
            CONTAMINATION_ERROR_STATUS, CONTAMINATION_WARNING_STATUS, DAZZLE_STATUS,
            NO_REFLECTION_STATUS, REFLECTOR_STATUS, VALID_STATUS,
        };
        use crate::data_output::measurement_data::MeasurementDataStatus;

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
        use crate::data_output::measurement_data::measurement_data_status_tests::{
            CONTAMINATION_ERROR_STATUS, CONTAMINATION_WARNING_STATUS, DAZZLE_STATUS,
            NO_REFLECTION_STATUS, REFLECTOR_STATUS, VALID_STATUS,
        };
        use crate::data_output::measurement_data::MeasurementDataStatus;

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
    use crate::data_output::measurement_data::{
        Measurement, MeasurementDataBlock, MeasurementDataStatus,
    };

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
