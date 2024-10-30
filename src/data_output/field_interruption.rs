use std::ops::Range;

use bitflags::bitflags;

#[derive(Debug, PartialEq)]
pub struct FieldInterruptions {
    cut_off_paths: Vec<CutOffPath>,
}

impl FieldInterruptions {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == 0 {
            return None;
        }

        let mut cut_off_paths: Vec<CutOffPath> = Vec::new();

        const U32_BYTES_NUMBER: usize = 4;
        const NUMBER_OF_CUT_OFF_PATHS: i32 = 24;
        let mut start_of_cut_off_path: usize = 0;

        for _ in 1..=NUMBER_OF_CUT_OFF_PATHS {
            let number_of_flag_bytes: Range<usize> =
                start_of_cut_off_path..start_of_cut_off_path + U32_BYTES_NUMBER;
            let number_of_flag_bytes: usize =
                u32::from_le_bytes(bytes[number_of_flag_bytes].try_into().unwrap()) as usize;

            let end_of_cut_off_path_bytes =
                start_of_cut_off_path + U32_BYTES_NUMBER + number_of_flag_bytes;

            let flag_bytes: Range<usize> =
                start_of_cut_off_path + U32_BYTES_NUMBER..end_of_cut_off_path_bytes;
            let flag_bytes = &bytes[flag_bytes];

            cut_off_paths.push(CutOffPath::from_bytes(flag_bytes));

            start_of_cut_off_path = end_of_cut_off_path_bytes;
        }

        Some(Self { cut_off_paths })
    }

    pub fn cut_off_path(&self, index: usize) -> Result<&CutOffPath, String> {
        let cut_off_path = self.cut_off_paths.get(index);
        match cut_off_path {
            Some(cut_off_path) => Ok(cut_off_path),
            None => Err(format!("index {} is out of range!", index).to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct CutOffPath {
    beams: Vec<Beam>,
}

impl CutOffPath {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut field_interruptions: Vec<Beam> = Vec::new();

        for byte in bytes {
            let byte = FieldInterruptionFlags::from_bits_truncate(*byte);

            let bits: Vec<FieldInterruptionFlags> = vec![
                FieldInterruptionFlags::BIT_0,
                FieldInterruptionFlags::BIT_1,
                FieldInterruptionFlags::BIT_2,
                FieldInterruptionFlags::BIT_3,
                FieldInterruptionFlags::BIT_4,
                FieldInterruptionFlags::BIT_5,
                FieldInterruptionFlags::BIT_6,
                FieldInterruptionFlags::BIT_7,
            ];

            for bit in bits {
                if byte.contains(bit) {
                    field_interruptions.push(Beam::Interrupted);
                } else {
                    field_interruptions.push(Beam::NotInterrupted);
                }
            }
        }
        Self {
            beams: field_interruptions,
        }
    }

    pub fn beam(&self, index: usize) -> Result<&Beam, String> {
        let beam = self.beams.get(index);
        match beam {
            Some(beam) => Ok(beam),
            None => Err(format!("index {} is out of range!", index).to_string()),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum Beam {
    Interrupted,
    NotInterrupted,
}

bitflags! {
    struct FieldInterruptionFlags: u8 {
        const BIT_0 = 0b0000_0001;
        const BIT_1 = 0b0000_0010;
        const BIT_2 = 0b0000_0100;
        const BIT_3 = 0b0000_1000;
        const BIT_4 = 0b0001_0000;
        const BIT_5 = 0b0010_0000;
        const BIT_6 = 0b0100_0000;
        const BIT_7 = 0b1000_0000;
    }
}

#[cfg(test)]
mod field_interruption_tests {
    use crate::data_output::field_interruption::{Beam, CutOffPath, FieldInterruptions};

    #[test]
    fn parse_field_interruptions_from_valid_block() {
        let (test_data, expected_block) = create_valid_test_data();
        let result = FieldInterruptions::from_bytes(&test_data).unwrap();
        assert_eq!(result, expected_block)
    }

    #[test]
    fn call_parsing_on_empty_block() {
        let test_data = [0u8; 3];
        let test_data = &test_data[0..0];
        let result = FieldInterruptions::from_bytes(&test_data);
        assert_eq!(result, None)
    }

    fn create_valid_test_data() -> ([u8; 168], FieldInterruptions) {
        let number_of_bytes: Vec<u8> = 3u32.to_le_bytes().iter().map(|byte| *byte).collect();

        let flags_variation_1: Vec<u8> = vec![0b00101101, 0b10010010, 0b01000001];
        let cut_off_path_variation_1 = CutOffPath {
            beams: vec![
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
            ],
        };

        let flags_variation_2: Vec<u8> = vec![0b10101010, 0b10000001, 0b00000000];
        let cut_off_path_variation_2 = CutOffPath {
            beams: vec![
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::Interrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
                Beam::NotInterrupted,
            ],
        };

        let mut test_data: Vec<u8> = Vec::new();
        let mut cut_off_paths: Vec<CutOffPath> = Vec::new();

        for i in 1..=24 {
            if i < 12 {
                test_data.extend(number_of_bytes.clone());
                test_data.extend(flags_variation_1.clone());
                cut_off_paths.push(cut_off_path_variation_1.clone());
            } else {
                test_data.extend(number_of_bytes.clone());
                test_data.extend(flags_variation_2.clone());
                cut_off_paths.push(cut_off_path_variation_2.clone());
            }
        }

        let expected_field_interruptions = FieldInterruptions { cut_off_paths };

        (test_data.try_into().unwrap(), expected_field_interruptions)
    }
}
