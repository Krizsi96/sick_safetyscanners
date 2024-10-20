#[derive(Copy, Clone, Debug)]
struct Block {
    offset: u16,
    size: u16,
}

#[derive(Debug)]
pub struct DataOutputHeader {
    version: u8,
    version_major: u8,
    version_minor: u8,
    release: u8,
    device_serial_number: u32,
    system_plug_serial_number: u32,
    channel_number: u8,
    sequence_number: u32,
    scan_number: u32,
    time_stamp_date: u16,
    time_stamp_time: u32,
    device_status_block: Block,
    output_configuration_block: Block,
    measurement_data_block: Block,
    field_interruption_block: Block,
    application_data_block: Block,
    local_ios_block: Block,
}

impl DataOutputHeader {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        DataOutputHeader {
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

#[cfg(test)]
mod data_output_header_tests {
    use crate::data_output::{Block, DataOutputHeader};
    use array_concat::concat_arrays;

    #[test]
    fn parse_version_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(result.version, expected_header.version);
    }

    #[test]
    fn parse_version_major_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(result.version_major, expected_header.version_major);
    }

    #[test]
    fn parse_version_minor_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(result.version_minor, expected_header.version_minor);
    }

    #[test]
    fn parse_release_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(result.release, expected_header.release);
    }

    #[test]
    fn parse_device_serial_number_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(
            result.device_serial_number,
            expected_header.device_serial_number
        );
    }

    #[test]
    fn parse_system_plug_serial_number_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(
            result.system_plug_serial_number,
            expected_header.system_plug_serial_number
        );
    }

    #[test]
    fn parse_channel_number_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(result.channel_number, expected_header.channel_number);
    }

    #[test]
    fn parse_sequence_number_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(result.sequence_number, expected_header.sequence_number);
    }

    #[test]
    fn parse_scan_number_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(result.scan_number, expected_header.scan_number);
    }

    #[test]
    fn parse_time_stamp_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(result.time_stamp_date, expected_header.time_stamp_date);
        assert_eq!(result.time_stamp_time, expected_header.time_stamp_time);
    }

    #[test]
    fn parse_device_status_block_info_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(
            result.device_status_block.offset,
            expected_header.device_status_block.offset
        );
        assert_eq!(
            result.device_status_block.size,
            expected_header.device_status_block.size
        );
    }

    #[test]
    fn parse_output_configuration_block_info_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(
            result.output_configuration_block.offset,
            expected_header.output_configuration_block.offset
        );
        assert_eq!(
            result.output_configuration_block.size,
            expected_header.output_configuration_block.size
        );
    }

    #[test]
    fn parse_measurement_data_block_info_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(
            result.measurement_data_block.offset,
            expected_header.measurement_data_block.offset
        );
        assert_eq!(
            result.measurement_data_block.size,
            expected_header.measurement_data_block.size
        );
    }

    #[test]
    fn parse_field_interruption_block_info_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(
            result.field_interruption_block.offset,
            expected_header.field_interruption_block.offset
        );
        assert_eq!(
            result.field_interruption_block.size,
            result.field_interruption_block.size
        );
    }

    #[test]
    fn parse_application_data_block_info_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(
            result.application_data_block.offset,
            expected_header.application_data_block.offset
        );
        assert_eq!(
            result.application_data_block.size,
            expected_header.application_data_block.size
        );
    }

    #[test]
    fn parse_local_ios_block_info_from_valid_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = DataOutputHeader::from_bytes(&test_data);
        assert_eq!(
            result.local_ios_block.offset,
            expected_header.local_ios_block.offset
        );
        assert_eq!(
            result.local_ios_block.size,
            expected_header.local_ios_block.size
        );
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
                [0; 8]
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
