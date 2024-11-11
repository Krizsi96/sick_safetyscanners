#[derive(Debug, PartialEq)]
pub struct DeviceStatus {
    pub status_of_safety_function: bool,
    pub status_sleep_mode: bool,
    pub contamination_warning: bool,
    pub contamination_error: bool,
    pub reference_contour_monitoring: bool,
    pub manipulation: bool,
    pub non_safe_cut_off_path: [bool; 8],
    pub safety_cut_off_path: [bool; 8],
    pub reset_required_cut_off_path: [bool; 8],
    pub current_monitoring_case_table_1: u8,
    pub current_monitoring_case_table_2: u8,
    pub device_error: bool,
    pub application_error: bool,
}

impl DeviceStatus {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let byte_0_flags = Byte0Flags::from_bits_truncate(bytes[0]);
        let byte_1_flags = Byte1Flags::from_bits_truncate(bytes[1]);
        let byte_4_flags = Byte4Flags::from_bits_truncate(bytes[4]);
        let byte_7_flags = Byte7Flags::from_bits_truncate(bytes[7]);
        let byte_15_flags = Byte15Flags::from_bits_truncate(bytes[15]);

        let status_of_safety_function =
            byte_0_flags.contains(Byte0Flags::STATUS_OF_SAFETY_FUNCTION);
        let status_sleep_mode = byte_0_flags.contains(Byte0Flags::STATUS_SLEEP_MODE);
        let contamination_warning = byte_0_flags.contains(Byte0Flags::CONTAMINATION_WARNING);
        let contamination_error = byte_0_flags.contains(Byte0Flags::CONTAMINATION_ERROR);
        let reference_contour_monitoring =
            byte_0_flags.contains(Byte0Flags::REFERENCE_CONTOUR_MONITORING);
        let manipulation = byte_0_flags.contains(Byte0Flags::MANIPULATION);

        let non_safe_cut_off_path = [
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_01),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_02),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_03),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_04),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_05),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_06),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_07),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_08),
        ];

        let safety_cut_off_path = [
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_01),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_02),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_03),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_04),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_05),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_06),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_07),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_08),
        ];

        let reset_required_cut_off_path = [
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_01),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_02),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_03),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_04),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_05),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_06),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_07),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_08),
        ];

        let current_monitoring_case_table_1 = bytes[10];
        let current_monitoring_case_table_2 = bytes[11];

        let device_error = byte_15_flags.contains(Byte15Flags::DEVICE_ERROR);
        let application_error = byte_15_flags.contains(Byte15Flags::APPLICATION_ERROR);

        Self {
            status_of_safety_function,
            status_sleep_mode,
            contamination_warning,
            contamination_error,
            reference_contour_monitoring,
            manipulation,
            non_safe_cut_off_path,
            safety_cut_off_path,
            reset_required_cut_off_path,
            current_monitoring_case_table_1,
            current_monitoring_case_table_2,
            device_error,
            application_error,
        }
    }
}

use bitflags::bitflags;
bitflags! {
    struct Byte0Flags: u8 {
        const STATUS_OF_SAFETY_FUNCTION = 0b0000_0001;
        const STATUS_SLEEP_MODE = 0b0000_0010;
        const CONTAMINATION_WARNING = 0b0000_0100;
        const CONTAMINATION_ERROR = 0b0000_1000;
        const REFERENCE_CONTOUR_MONITORING = 0b0001_0000;
        const MANIPULATION = 0b0010_0000;
    }

    struct Byte1Flags: u8 {
        const NON_SAFE_CUT_OFF_PATH_01 = 0b0000_0001;
        const NON_SAFE_CUT_OFF_PATH_02 = 0b0000_0010;
        const NON_SAFE_CUT_OFF_PATH_03 = 0b0000_0100;
        const NON_SAFE_CUT_OFF_PATH_04 = 0b0000_1000;
        const NON_SAFE_CUT_OFF_PATH_05 = 0b0001_0000;
        const NON_SAFE_CUT_OFF_PATH_06 = 0b0010_0000;
        const NON_SAFE_CUT_OFF_PATH_07 = 0b0100_0000;
        const NON_SAFE_CUT_OFF_PATH_08 = 0b1000_0000;
    }

    struct Byte4Flags: u8 {
        const SAFETY_CUT_OFF_PATH_01 = 0b0000_0001;
        const SAFETY_CUT_OFF_PATH_02 = 0b0000_0010;
        const SAFETY_CUT_OFF_PATH_03 = 0b0000_0100;
        const SAFETY_CUT_OFF_PATH_04 = 0b0000_1000;
        const SAFETY_CUT_OFF_PATH_05 = 0b0001_0000;
        const SAFETY_CUT_OFF_PATH_06 = 0b0010_0000;
        const SAFETY_CUT_OFF_PATH_07 = 0b0100_0000;
        const SAFETY_CUT_OFF_PATH_08 = 0b1000_0000;
    }

    struct Byte7Flags: u8 {
        const RESET_REQUIRED_CUT_OFF_PATH_01 = 0b0000_0001;
        const RESET_REQUIRED_CUT_OFF_PATH_02 = 0b0000_0010;
        const RESET_REQUIRED_CUT_OFF_PATH_03 = 0b0000_0100;
        const RESET_REQUIRED_CUT_OFF_PATH_04 = 0b0000_1000;
        const RESET_REQUIRED_CUT_OFF_PATH_05 = 0b0001_0000;
        const RESET_REQUIRED_CUT_OFF_PATH_06 = 0b0010_0000;
        const RESET_REQUIRED_CUT_OFF_PATH_07 = 0b0100_0000;
        const RESET_REQUIRED_CUT_OFF_PATH_08 = 0b1000_0000;
    }

    struct Byte15Flags: u8 {
        const APPLICATION_ERROR = 0b0000_0001;
        const DEVICE_ERROR = 0b0000_0010;
    }
}

#[cfg(test)]
mod device_status_block_tests {
    use crate::data_output::device_status::DeviceStatus;
    use crate::data_output::device_status::{
        Byte0Flags, Byte15Flags, Byte1Flags, Byte4Flags, Byte7Flags,
    };

    #[test]
    fn parse_from_valid_block() {
        let (test_data, expected_block) = create_valid_test_data();
        let result = DeviceStatus::from_bytes(&test_data);
        assert_eq!(result, expected_block)
    }

    fn create_valid_test_data() -> ([u8; 16], DeviceStatus) {
        let byte_0_flags = Byte0Flags::STATUS_OF_SAFETY_FUNCTION
            | Byte0Flags::CONTAMINATION_WARNING
            | Byte0Flags::REFERENCE_CONTOUR_MONITORING;
        let byte_1_flags = Byte1Flags::NON_SAFE_CUT_OFF_PATH_02
            | Byte1Flags::NON_SAFE_CUT_OFF_PATH_04
            | Byte1Flags::NON_SAFE_CUT_OFF_PATH_06
            | Byte1Flags::NON_SAFE_CUT_OFF_PATH_08;
        let byte_4_flags = Byte4Flags::SAFETY_CUT_OFF_PATH_01
            | Byte4Flags::SAFETY_CUT_OFF_PATH_04
            | Byte4Flags::SAFETY_CUT_OFF_PATH_07;
        let byte_7_flags = Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_02
            | Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_05
            | Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_08;
        let byte_15_flags = Byte15Flags::APPLICATION_ERROR;

        let status_of_safety_function =
            byte_0_flags.contains(Byte0Flags::STATUS_OF_SAFETY_FUNCTION);
        let status_sleep_mode = byte_0_flags.contains(Byte0Flags::STATUS_SLEEP_MODE);
        let contamination_warning = byte_0_flags.contains(Byte0Flags::CONTAMINATION_WARNING);
        let contamination_error = byte_0_flags.contains(Byte0Flags::CONTAMINATION_ERROR);
        let reference_contour_monitoring =
            byte_0_flags.contains(Byte0Flags::REFERENCE_CONTOUR_MONITORING);
        let manipulation = byte_0_flags.contains(Byte0Flags::MANIPULATION);

        let non_safe_cut_off_path = [
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_01),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_02),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_03),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_04),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_05),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_06),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_07),
            byte_1_flags.contains(Byte1Flags::NON_SAFE_CUT_OFF_PATH_08),
        ];

        let safety_cut_off_path = [
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_01),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_02),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_03),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_04),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_05),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_06),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_07),
            byte_4_flags.contains(Byte4Flags::SAFETY_CUT_OFF_PATH_08),
        ];

        let reset_required_cut_off_path = [
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_01),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_02),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_03),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_04),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_05),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_06),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_07),
            byte_7_flags.contains(Byte7Flags::RESET_REQUIRED_CUT_OFF_PATH_08),
        ];

        let device_error = byte_15_flags.contains(Byte15Flags::DEVICE_ERROR);
        let application_error = byte_15_flags.contains(Byte15Flags::APPLICATION_ERROR);

        let current_monitoring_case_table_1 = 128u8;
        let current_monitoring_case_table_2 = 255u8;
        (
            [
                byte_0_flags.bits(),
                byte_1_flags.bits(),
                0,
                0,
                byte_4_flags.bits(),
                0,
                0,
                byte_7_flags.bits(),
                0,
                0,
                current_monitoring_case_table_1,
                current_monitoring_case_table_2,
                0,
                0,
                0,
                byte_15_flags.bits(),
            ],
            DeviceStatus {
                status_of_safety_function,
                status_sleep_mode,
                contamination_warning,
                contamination_error,
                reference_contour_monitoring,
                manipulation,
                non_safe_cut_off_path,
                safety_cut_off_path,
                reset_required_cut_off_path,
                current_monitoring_case_table_1,
                current_monitoring_case_table_2,
                device_error,
                application_error,
            },
        )
    }
}
