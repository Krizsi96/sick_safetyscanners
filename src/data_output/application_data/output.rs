use bitflags::bitflags;

use crate::data_output::application_data::output::Message::{
    ContaminationError, ContaminationWarning, CriticalError, Dazzle, Manipulation,
    ReferenceContourMonitoring,
};
use crate::data_output::application_data::output::Safety::{NonSafe, Safe};
use crate::data_output::application_data::output::StandbyState::{
    DeviceIsInStandby, DeviceIsNotInStandby,
};
use crate::data_output::application_data::output::Status::{Clear, Triggered};
use crate::data_output::application_data::output::Validity::{Invalid, Valid};
use crate::data_output::application_data::{NUMBER_OF_CUT_OFF_PATHS, NUMBER_OF_MONITORING_CASES};
use crate::data_output::bit_flags::bit_status_in_bytes;

#[derive(PartialEq, Debug)]
pub struct Outputs {
    cut_off_paths: [CutOffPath; NUMBER_OF_CUT_OFF_PATHS],
    monitoring_case_number: [MonitoringCaseNumber; NUMBER_OF_MONITORING_CASES],
    standby_state: StandbyState,
    messages: Vec<Message>,
    flags: Flags,
}

impl Outputs {
    fn from_bytes(bytes: &[u8]) -> Self {
        Outputs {
            cut_off_paths: CutOffPath::from_bytes(&bytes[0..12]),
            monitoring_case_number: MonitoringCaseNumber::get_cases_from_bytes(&bytes[12..56]),
            standby_state: StandbyState::from_byte(bytes[56]),
            messages: Message::get_messages_from_byte(bytes[57]),
            flags: Flags::from_byte(bytes[123]),
        }
    }
}

#[derive(PartialEq, Debug)]
struct CutOffPath {
    logic_status: Status,
    safety: Safety,
    validity: Validity,
}

impl CutOffPath {
    fn from_bytes(bytes: &[u8]) -> [CutOffPath; NUMBER_OF_CUT_OFF_PATHS] {
        let mut cut_off_paths: Vec<CutOffPath> = Vec::new();

        for index in 0..NUMBER_OF_CUT_OFF_PATHS {
            cut_off_paths.push(CutOffPath {
                logic_status: Status::from_bool(bit_status_in_bytes(index, &bytes[0..4])),
                safety: Safety::from_bool(bit_status_in_bytes(index, &bytes[4..8])),
                validity: Validity::from_bool(bit_status_in_bytes(index, &bytes[8..12])),
            })
        }

        cut_off_paths.try_into().unwrap()
    }
}

#[derive(PartialEq, Debug)]
enum Safety {
    Safe,
    NonSafe,
}

impl Safety {
    fn from_bool(safety: bool) -> Self {
        match safety {
            true => Safe,
            false => NonSafe,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Status {
    Triggered,
    Clear,
}

impl Status {
    fn from_bool(status: bool) -> Self {
        match status {
            true => Triggered,
            false => Clear,
        }
    }
}

#[derive(PartialEq, Debug)]
struct MonitoringCaseNumber {
    number: u16,
    validity: Validity,
}

impl MonitoringCaseNumber {
    fn get_cases_from_bytes(bytes: &[u8]) -> [Self; NUMBER_OF_MONITORING_CASES] {
        let mut monitoring_case_numbers: Vec<Self> = Vec::with_capacity(NUMBER_OF_MONITORING_CASES);

        for index in 0..NUMBER_OF_MONITORING_CASES {
            let case_number_index = 2 * index;
            let monitoring_case_number = u16::from_le_bytes(
                bytes[0 + case_number_index..2 + case_number_index]
                    .try_into()
                    .unwrap(),
            );

            monitoring_case_numbers.push(MonitoringCaseNumber {
                number: monitoring_case_number,
                validity: Validity::from_bool(bit_status_in_bytes(index, &bytes[40..44])),
            })
        }

        monitoring_case_numbers.try_into().unwrap()
    }
}

#[derive(PartialEq, Debug)]
enum StandbyState {
    DeviceIsInStandby,
    DeviceIsNotInStandby,
}

impl StandbyState {
    fn from_byte(byte: u8) -> Self {
        match byte {
            1 => DeviceIsInStandby,
            2 => DeviceIsNotInStandby,
            _ => panic!("Unknown standby state: {byte}"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Message {
    ContaminationWarning,
    ContaminationError,
    Manipulation,
    Dazzle,
    ReferenceContourMonitoring,
    CriticalError,
}

impl Message {
    fn get_messages_from_byte(byte: u8) -> Vec<Self> {
        (0..u8::BITS as usize)
            .filter(|index| bit_status_in_bytes(*index, &[byte]))
            .filter_map(|bit| match bit {
                0 => Some(ContaminationWarning),
                1 => Some(ContaminationError),
                2 => Some(Manipulation),
                3 => Some(Dazzle),
                4 => Some(ReferenceContourMonitoring),
                5 => Some(CriticalError),
                _ => None,
            })
            .collect()
    }
}

#[derive(PartialEq, Debug)]
struct Flags {
    sleep_mode_status: Validity,
    messages_output: Validity,
}

impl Flags {
    fn from_byte(byte: u8) -> Self {
        let byte = StatusFlags::from_bits_truncate(byte);
        Self {
            sleep_mode_status: Validity::from_bool(
                byte.contains(StatusFlags::SLEEP_MODE_STATUS_OUTPUT_IS_VALID),
            ),
            messages_output: Validity::from_bool(
                byte.contains(StatusFlags::MESSAGES_OUTPUT_IS_VALID),
            ),
        }
    }
}

bitflags! {
    struct StatusFlags: u8 {
        const SLEEP_MODE_STATUS_OUTPUT_IS_VALID = 0b0000_0001;
        const MESSAGES_OUTPUT_IS_VALID = 0b0000_0010;
    }
}

#[derive(PartialEq, Debug)]
enum Validity {
    Valid,
    Invalid,
}

impl Validity {
    fn from_bool(status: bool) -> Self {
        match status {
            true => Valid,
            false => Invalid,
        }
    }
}

#[cfg(test)]
mod application_data_outputs_tests {
    use array_concat::concat_arrays;

    use crate::data_output::application_data::output::cut_off_paths_tests;
    use crate::data_output::application_data::output::monitoring_case_number_tests;
    use crate::data_output::application_data::output::Message::{
        ContaminationWarning, Dazzle, Manipulation, ReferenceContourMonitoring,
    };
    use crate::data_output::application_data::output::Validity::Valid;
    use crate::data_output::application_data::output::{Flags, Outputs, StandbyState};

    #[test]
    fn parse_application_data_outputs_from_bytes() {
        let (test_data, expected_application_data_outputs) = create_valid_test_data();
        let result = Outputs::from_bytes(&test_data);
        assert_eq!(result, expected_application_data_outputs);
    }

    fn create_valid_test_data() -> ([u8; 124], Outputs) {
        let (cut_off_path_data, cut_off_path_expected) =
            cut_off_paths_tests::create_valid_test_data();
        let (monitoring_cases_data, monitoring_cases_expected) =
            monitoring_case_number_tests::create_valid_test_data();
        let standby_state = [1u8];
        let messages = [0b1001_1101];
        let flags = [0b0000_0011];

        let test_data = concat_arrays!(
            cut_off_path_data,
            monitoring_cases_data,
            standby_state,
            messages,
            [0u8; 65],
            flags
        );

        let expected_application_data_outputs = Outputs {
            cut_off_paths: cut_off_path_expected,
            monitoring_case_number: monitoring_cases_expected,
            standby_state: StandbyState::DeviceIsInStandby,
            messages: vec![
                ContaminationWarning,
                Manipulation,
                Dazzle,
                ReferenceContourMonitoring,
            ],
            flags: Flags {
                sleep_mode_status: Valid,
                messages_output: Valid,
            },
        };

        (test_data, expected_application_data_outputs)
    }
}

#[cfg(test)]
mod standby_state_tests {
    use crate::data_output::application_data::output::StandbyState;

    #[test]
    fn device_is_in_standby() {
        let standby = 1u8;
        let result = StandbyState::from_byte(standby);
        assert_eq!(result, StandbyState::DeviceIsInStandby);
    }

    #[test]
    fn device_is_not_in_standby() {
        let not_standby = 2u8;
        let result = StandbyState::from_byte(not_standby);
        assert_eq!(result, StandbyState::DeviceIsNotInStandby);
    }

    #[test]
    #[should_panic]
    fn unknown_state() {
        let unknown_state = 3u8;
        StandbyState::from_byte(unknown_state);
    }
}

#[cfg(test)]
mod messages_tests {
    use crate::data_output::application_data::output::Message;
    use crate::data_output::application_data::output::Message::{
        ContaminationError, ContaminationWarning, CriticalError, Dazzle, Manipulation,
        ReferenceContourMonitoring,
    };

    #[test]
    fn no_messages() {
        let messages = 0b0000_0000;
        let result = Message::get_messages_from_byte(messages);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn reserved_bits_are_ignored() {
        let messages = 0b1100_0000;
        let result = Message::get_messages_from_byte(messages);
        assert_eq!(result.len(), 0);
    }

    macro_rules! tests_for_messages {
        ($($name:ident: input: $input:expr, expected: $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let input: u8 = $input;
                    let expected: Message = $expected;
                    let result = Message::get_messages_from_byte(input);
                    assert_eq!(result[0], expected);
                }
            )*
        };
    }

    tests_for_messages! {
        contamination_warning: input: 0b0000_0001 , expected: ContaminationWarning,
        contamination_error: input: 0b0000_0010, expected: ContaminationError,
        manipulation: input: 0b0000_0100, expected: Manipulation,
        dazzle: input: 0b0000_1000, expected: Dazzle,
        reference_contour_monitoring: input: 0b0001_0000, expected: ReferenceContourMonitoring,
        critical_error: input: 0b0010_0000, expected: CriticalError,
    }

    #[test]
    fn multiple_messages() {
        let messages = 0b1111_1111;
        let result = Message::get_messages_from_byte(messages);
        assert_eq!(
            result,
            vec![
                ContaminationWarning,
                ContaminationError,
                Manipulation,
                Dazzle,
                ReferenceContourMonitoring,
                CriticalError
            ]
        )
    }
}

#[cfg(test)]
mod flags_tests {
    use crate::data_output::application_data::output::Flags;
    use crate::data_output::application_data::output::Validity::{Invalid, Valid};

    macro_rules! tests_for_flags {
        ($($name:ident: input: $input:expr, expected: $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let input: u8 = $input;
                    let expected: Flags = $expected;
                    let result = Flags::from_byte(input);
                    assert_eq!(result, expected);
                }
            )*
        };
    }

    tests_for_flags! {
        sleep_mode_and_messages_invalid:
            input: 0b0000_0000,
            expected: Flags {
                sleep_mode_status: Invalid,
                messages_output: Invalid,
            },
        sleep_mode_valid_and_messages_invalid:
            input: 0b0000_0001,
            expected: Flags {
                sleep_mode_status: Valid,
                messages_output: Invalid,
            },
        sleep_mode_invalid_and_messages_valid:
            input: 0b0000_0010,
            expected: Flags {
                sleep_mode_status: Invalid,
                messages_output: Valid,
            },
        sleep_mode_valid_and_messages_valid:
            input: 0b0000_0011,
            expected: Flags {
                sleep_mode_status: Valid,
                messages_output: Valid,
            },
    }
}

#[cfg(test)]
mod monitoring_case_number_tests {
    use array_concat::concat_arrays;

    use crate::data_output::application_data::output::MonitoringCaseNumber;
    use crate::data_output::application_data::output::Validity::{Invalid, Valid};
    use crate::data_output::application_data::NUMBER_OF_MONITORING_CASES;

    #[test]
    fn monitoring_case_numbers_from_bytes() {
        let (test_data, expected) = create_valid_test_data();
        let result = MonitoringCaseNumber::get_cases_from_bytes(&test_data);
        assert_eq!(result, expected)
    }

    macro_rules! monitoring_cases {
        ( $( { number: $number:expr, validity: $validity:expr } ),* $(,)?) => {
            [
                $(
                    MonitoringCaseNumber {
                        number: $number,
                        validity: $validity,
                    }
                ),*
            ]
        }
    }

    macro_rules! monitoring_case_numbers_data {
        ( $( { number: $number:expr} ),* $(,)?) => {
            concat_arrays!(
                $(
                    $number.to_le_bytes()
                ),*
            )
        }
    }

    pub fn create_valid_test_data() -> ([u8; 44], [MonitoringCaseNumber; NUMBER_OF_MONITORING_CASES])
    {
        let monitoring_case_number_bytes: [u8; 40] = monitoring_case_numbers_data!(
            { number: 1u16 }, { number: 2u16 }, { number: 3u16 }, { number: 4u16 },
            { number: 5u16 }, { number: 6u16 }, { number: 7u16 }, { number: 8u16 },
            { number: 9u16 }, { number: 10u16 }, { number: 11u16 }, { number: 12u16 },
            { number: 13u16 }, { number: 14u16 }, { number: 15u16 }, { number: 16u16 },
            { number: 17u16 }, { number: 18u16 }, { number: 19u16 }, { number: 20u16 },
        );

        let monitoring_case_validity_flags: [u8; 4] =
            [0b0101_0101, 0b1010_1010u8, 0b0000_0000, 0b0000_0000];
        let test_data: [u8; 44] =
            concat_arrays!(monitoring_case_number_bytes, monitoring_case_validity_flags);

        let expected_monitoring_cases: [MonitoringCaseNumber; NUMBER_OF_MONITORING_CASES] = monitoring_cases! [
        { number: 1, validity: Valid },
        { number: 2, validity: Invalid },
        { number: 3, validity: Valid },
        { number: 4, validity: Invalid },
        { number: 5, validity: Valid },
        { number: 6, validity: Invalid },
        { number: 7, validity: Valid },
        { number: 8, validity: Invalid },
        { number: 9, validity: Invalid },
        { number: 10, validity: Valid },
        { number: 11, validity: Invalid },
        { number: 12, validity: Valid },
        { number: 13, validity: Invalid },
        { number: 14, validity: Valid },
        { number: 15, validity: Invalid },
        { number: 16, validity: Valid },
        { number: 17, validity: Invalid },
        { number: 18, validity: Invalid },
        { number: 19, validity: Invalid },
        { number: 20, validity: Invalid },
        ];

        (test_data, expected_monitoring_cases)
    }
}

#[cfg(test)]
mod cut_off_paths_tests {
    use array_concat::concat_arrays;

    use crate::data_output::application_data::output::CutOffPath;
    use crate::data_output::application_data::output::Safety::{NonSafe, Safe};
    use crate::data_output::application_data::output::Status::{Clear, Triggered};
    use crate::data_output::application_data::output::Validity::{Invalid, Valid};
    use crate::data_output::application_data::NUMBER_OF_CUT_OFF_PATHS;

    #[test]
    fn cut_off_paths_from_bytes() {
        let (test_data, expected) = create_valid_test_data();
        let result = CutOffPath::from_bytes(&test_data);
        assert_eq!(result, expected);
    }

    macro_rules! cut_off_paths {
        ( $( { logic: $logic:expr, safety: $safety:expr, validity: $validity:expr } ),* $(,)?) => {
            [
                $(
                    CutOffPath {
                        logic_status: $logic,
                        safety: $safety,
                        validity: $validity,
                    }
                ),*
            ]
        }
    }

    pub fn create_valid_test_data() -> ([u8; 12], [CutOffPath; NUMBER_OF_CUT_OFF_PATHS]) {
        let logic_state = [0b0101_0101, 0b1010_1010, 0b1001_0110, 0b0000_0000];
        let safety = [0b0101_0101, 0b1010_1010, 0b1001_0110, 0b0000_0000];
        let validity = [0b0101_0101, 0b1010_1010, 0b1001_0110, 0b0000_0000];

        let test_data: [u8; 12] = concat_arrays!(logic_state, safety, validity);

        let expected_cut_off_paths: [CutOffPath; NUMBER_OF_CUT_OFF_PATHS] = cut_off_paths!(
            { logic: Triggered, safety: Safe, validity: Valid },
            { logic: Clear, safety: NonSafe, validity: Invalid },
            { logic: Triggered, safety: Safe, validity: Valid },
            { logic: Clear, safety: NonSafe, validity: Invalid },
            { logic: Triggered, safety: Safe, validity: Valid },
            { logic: Clear, safety: NonSafe, validity: Invalid },
            { logic: Triggered, safety: Safe, validity: Valid },
            { logic: Clear, safety: NonSafe, validity: Invalid },
            { logic: Clear, safety: NonSafe, validity: Invalid },
            { logic: Triggered, safety: Safe, validity: Valid },
            { logic: Clear, safety: NonSafe, validity: Invalid },
            { logic: Triggered, safety: Safe, validity: Valid },
            { logic: Clear, safety: NonSafe, validity: Invalid },
            { logic: Triggered, safety: Safe, validity: Valid },
            { logic: Clear, safety: NonSafe, validity: Invalid },
            { logic: Triggered, safety: Safe, validity: Valid },
            { logic: Clear, safety: NonSafe, validity: Invalid },
            { logic: Triggered, safety: Safe, validity: Valid },
            { logic: Triggered, safety: Safe, validity: Valid },
            { logic: Clear, safety: NonSafe, validity: Invalid },
        );

        (test_data, expected_cut_off_paths)
    }
}
