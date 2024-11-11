use crate::data_output::application_data::NUMBER_OF_MONITORING_CASES;
use crate::data_output::bit_flags::bit_status_in_bytes;
use bitflags::bitflags;

#[derive(PartialEq, Debug)]
pub struct Inputs {
    static_control_inputs: [StaticControlInput; u32::BITS as usize],
    monitoring_case_numbers: [MonitoringCaseNumber; NUMBER_OF_MONITORING_CASES],
    dynamic_control_inputs: DynamicControlInputs,
    standby_state_input: StandbyStateInput,
}

impl Inputs {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            static_control_inputs: StaticControlInput::from_bytes(&bytes[0..12]),
            monitoring_case_numbers: MonitoringCaseNumber::from_bytes(&bytes[12..56]),
            dynamic_control_inputs: DynamicControlInputs::from_bytes(&bytes[56..62]),
            standby_state_input: StandbyStateInput::from_byte(bytes[74]),
        }
    }
}

#[derive(PartialEq, Debug)]
struct StaticControlInput {
    status: bool,
    switchover: Switchover,
}

impl StaticControlInput {
    fn from_bytes(bytes: &[u8]) -> [Self; u32::BITS as usize] {
        let mut static_control_input: Vec<Self> = Vec::with_capacity(u32::BITS as usize);

        for i in 0..u32::BITS as usize {
            static_control_input.push(StaticControlInput {
                status: bit_status_in_bytes(i, &bytes[0..4]),
                switchover: Switchover::from_bytes(i, &bytes[4..8]),
            })
        }

        static_control_input.try_into().unwrap()
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct MonitoringCaseNumber {
    monitoring_case_number: u16,
    switchover: Switchover,
}

impl MonitoringCaseNumber {
    fn from_bytes(bytes: &[u8]) -> [Self; NUMBER_OF_MONITORING_CASES] {
        let mut monitoring_case_numbers: Vec<Self> = Vec::with_capacity(NUMBER_OF_MONITORING_CASES);

        for i in 0..NUMBER_OF_MONITORING_CASES {
            let case_number_index = 2 * i;
            let monitoring_case_number = u16::from_le_bytes(
                bytes[0 + case_number_index..2 + case_number_index]
                    .try_into()
                    .unwrap(),
            );

            monitoring_case_numbers.push(MonitoringCaseNumber {
                monitoring_case_number,
                switchover: Switchover::from_bytes(i, &bytes[40..44]),
            })
        }

        monitoring_case_numbers.try_into().unwrap()
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Switchover {
    Available,
    NotAvailable,
}

impl Switchover {
    fn from_bytes(index: usize, bytes: &[u8]) -> Self {
        match bit_status_in_bytes(index, bytes) {
            true => Switchover::Available,
            false => Switchover::NotAvailable,
        }
    }
}

#[derive(PartialEq, Debug)]
struct DynamicControlInputs {
    speed_1: Speed,
    speed_2: Speed,
}

impl DynamicControlInputs {
    fn from_bytes(bytes: &[u8]) -> Self {
        let flags = ValidSpeedFlags::from_bits_truncate(bytes[4]);
        let speed_1 = Speed {
            value: i16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            valid: flags.contains(ValidSpeedFlags::SPEED_1),
        };
        let speed_2 = Speed {
            value: i16::from_le_bytes(bytes[2..4].try_into().unwrap()),
            valid: flags.contains(ValidSpeedFlags::SPEED_2),
        };
        Self { speed_1, speed_2 }
    }
}

bitflags! {
    struct ValidSpeedFlags: u8 {
        const SPEED_1 = 0b0000_0001;
        const SPEED_2 = 0b0000_0010;
    }
}

#[derive(PartialEq, Debug)]
struct Speed {
    value: i16,
    valid: bool,
}

#[derive(PartialEq, Debug)]
enum StandbyStateInput {
    HIGH,
    LOW,
}

impl StandbyStateInput {
    fn from_byte(byte: u8) -> Self {
        match byte {
            1 => StandbyStateInput::HIGH,
            2 => StandbyStateInput::LOW,
            _ => panic!("Invalid standby state input flag"),
        }
    }
}

#[cfg(test)]
mod application_data_tests {
    use array_concat::concat_arrays;

    use crate::data_output::application_data::input::{
        DynamicControlInputs, Inputs, MonitoringCaseNumber, Speed, StandbyStateInput,
        StaticControlInput, Switchover,
    };

    #[test]
    fn parse_application_data_from_bytes() {
        let (test_data, expected_application_data) = create_valid_test_data();
        let result = Inputs::from_bytes(&test_data);
        assert_eq!(result, expected_application_data);
    }

    fn create_valid_test_data() -> ([u8; 140], Inputs) {
        let control_input_bytes: [u8; 4] =
            [0b0110_1001u8, 0b1010_0000u8, 0b0000_0010u8, 0b0011_1100u8];

        let control_input_flag_bytes: [u8; 4] =
            [0b1001_0101u8, 0b1010_1100u8, 0b0101_0101u8, 0b0000_0001u8];

        let monitoring_case_number_bytes: [u8; 40] = concat_arrays!(
            1u16.to_le_bytes(),
            2u16.to_le_bytes(),
            3u16.to_le_bytes(),
            4u16.to_le_bytes(),
            5u16.to_le_bytes(),
            6u16.to_le_bytes(),
            7u16.to_le_bytes(),
            8u16.to_le_bytes(),
            9u16.to_le_bytes(),
            10u16.to_le_bytes(),
            11u16.to_le_bytes(),
            12u16.to_le_bytes(),
            13u16.to_le_bytes(),
            14u16.to_le_bytes(),
            15u16.to_le_bytes(),
            16u16.to_le_bytes(),
            17u16.to_le_bytes(),
            18u16.to_le_bytes(),
            19u16.to_le_bytes(),
            20u16.to_le_bytes()
        );

        let monitoring_case_switchover_flags: [u8; 4] =
            [0b0101_0101, 0b1010_1010u8, 0b0000_0000, 0b0000_0000];

        let dynamic_control_inputs: [u8; 6] = concat_arrays!(
            100u16.to_le_bytes(),
            200u16.to_le_bytes(),
            [0b0000_0010u8],
            [0u8]
        );

        let standby_state_input = [1u8];

        let test_data: [u8; 140] = concat_arrays!(
            control_input_bytes,
            control_input_flag_bytes,
            [0u8; 4],
            monitoring_case_number_bytes,
            monitoring_case_switchover_flags,
            dynamic_control_inputs,
            [0u8; 12],
            standby_state_input,
            [0u8; 65]
        );

        let expected_application_data = Inputs {
            static_control_inputs: [
                StaticControlInput {
                    status: true,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: true,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: true,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: true,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: true,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: true,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: true,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::Available,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: true,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: true,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: true,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: true,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
                StaticControlInput {
                    status: false,
                    switchover: Switchover::NotAvailable,
                },
            ],
            monitoring_case_numbers: [
                MonitoringCaseNumber {
                    monitoring_case_number: 1u16,
                    switchover: Switchover::Available,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 2u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 3u16,
                    switchover: Switchover::Available,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 4u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 5u16,
                    switchover: Switchover::Available,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 6u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 7u16,
                    switchover: Switchover::Available,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 8u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 9u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 10u16,
                    switchover: Switchover::Available,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 11u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 12u16,
                    switchover: Switchover::Available,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 13u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 14u16,
                    switchover: Switchover::Available,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 15u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 16u16,
                    switchover: Switchover::Available,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 17u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 18u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 19u16,
                    switchover: Switchover::NotAvailable,
                },
                MonitoringCaseNumber {
                    monitoring_case_number: 20u16,
                    switchover: Switchover::NotAvailable,
                },
            ],
            dynamic_control_inputs: DynamicControlInputs {
                speed_1: Speed {
                    value: 100,
                    valid: false,
                },
                speed_2: Speed {
                    value: 200,
                    valid: true,
                },
            },
            standby_state_input: StandbyStateInput::HIGH,
        };

        (test_data, expected_application_data)
    }
}
