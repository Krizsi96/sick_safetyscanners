use std::net::UdpSocket;
use std::ops::Range;

const DATAGRAM_MARKER_BYTES: Range<usize> = 0..4;
const PROTOCOL_BYTES: Range<usize> = 4..6;
const VERSION_MAJ_BYTE: usize = 6;
const VERSION_MIN_BYTE: usize = 7;
const TOTAL_LENGTH_BYTES: Range<usize> = 8..12;
const IDENTIFICATION_BYTES: Range<usize> = 12..16;
const FRAGMENT_OFFSET_BYTES: Range<usize> = 16..20;

#[derive(Debug)]
struct UDPDatagramHeader {
    datagram_marker: String,
    protocol: String,
    version_maj: u8,
    version_min: u8,
    total_length: u32,
    identification: u32,
    fragment_offset: u32,
}
fn parse_udp_datagram_header(buffer: &[u8]) -> UDPDatagramHeader {
    UDPDatagramHeader {
        datagram_marker: String::from_utf8(buffer[DATAGRAM_MARKER_BYTES].into()).unwrap(),
        protocol: String::from_utf8(buffer[PROTOCOL_BYTES].into()).unwrap(),
        version_maj: buffer[VERSION_MAJ_BYTE],
        version_min: buffer[VERSION_MIN_BYTE],
        total_length: u32::from_le_bytes(buffer[TOTAL_LENGTH_BYTES].try_into().unwrap()),
        identification: u32::from_le_bytes(buffer[IDENTIFICATION_BYTES].try_into().unwrap()),
        fragment_offset: u32::from_le_bytes(buffer[FRAGMENT_OFFSET_BYTES].try_into().unwrap()),
    }
}

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("192.168.1.15:1025")?;

        // Receives a single datagram message on the socket. If 'buf' is too small
        // to hold the message, it will be cut off.
        let mut buf = [0; 2048];
        let (message_size, source_address) = socket.recv_from(&mut buf)?;

        println!("received packet from {}", source_address);

        let datagram_header = parse_udp_datagram_header(&buf);
        println!("{:?}", datagram_header);

        let data_field = &buf[24..message_size];

        println!(
            "data field: {:?}",
            data_field
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<Vec<String>>()
        );

        if datagram_header.fragment_offset == 0 {
            let version = data_field[0];
            let maj_version = data_field[1];
            let min_version = data_field[2];
            let release = data_field[3];
            let serial_number = u32::from_le_bytes(data_field[4..8].try_into().unwrap());
            let system_plug_serial_number =
                u32::from_le_bytes(data_field[8..12].try_into().unwrap());
            let channel_number = data_field[12];
            let sequence_number = u32::from_le_bytes(data_field[16..20].try_into().unwrap());
            let scan_number = u32::from_le_bytes(data_field[20..24].try_into().unwrap());
            let time_stamp_date = u16::from_le_bytes(data_field[24..26].try_into().unwrap());
            let time_stamp_time = u32::from_le_bytes(data_field[28..32].try_into().unwrap());
            let block_device_status_offset =
                u16::from_le_bytes(data_field[32..34].try_into().unwrap());
            let block_device_status_size =
                u16::from_le_bytes(data_field[34..36].try_into().unwrap());
            let block_output_configuration_offset =
                u16::from_le_bytes(data_field[36..38].try_into().unwrap());
            let block_output_configuration_size =
                u16::from_le_bytes(data_field[38..40].try_into().unwrap());
            let block_measurement_data_offset =
                u16::from_le_bytes(data_field[40..42].try_into().unwrap());
            let block_measurement_data_size =
                u16::from_le_bytes(data_field[42..44].try_into().unwrap());
            let block_field_interruption_offset =
                u16::from_le_bytes(data_field[44..46].try_into().unwrap());
            let block_field_interruption_size =
                u16::from_le_bytes(data_field[46..48].try_into().unwrap());
            let block_application_data_offset =
                u16::from_le_bytes(data_field[48..50].try_into().unwrap());
            let block_application_data_size =
                u16::from_le_bytes(data_field[50..52].try_into().unwrap());
            let block_local_ios_offset = u16::from_le_bytes(data_field[52..54].try_into().unwrap());
            let block_local_ios_size = u16::from_le_bytes(data_field[54..56].try_into().unwrap());

            println!(
                "version: {version}\n\
            major version: {maj_version}\n\
            minor version: {min_version}\n\
            release: {release}\n\
            serial number: {serial_number}\n\
            system plug serial number: {system_plug_serial_number}\n\
            channel number: {channel_number}\n\
            sequence number: {sequence_number}\n\
            scan number: {scan_number}\n\
            time stamp date: {time_stamp_date}\n\
            time stamp time: {time_stamp_time}\n\
            block device status offset: {block_device_status_offset}\n\
            block device status size: {block_device_status_size}\n\
            block output configuration offset: {block_output_configuration_offset}\n\
            block output configuration size: {block_output_configuration_size}\n\
            block measurement data offset: {block_measurement_data_offset}\n\
            block measurement data size: {block_measurement_data_size}\n\
            block field interruption offset: {block_field_interruption_offset}\n\
            block field interruption size: {block_field_interruption_size}\n\
            block application data offset: {block_application_data_offset}\n\
            block application data size: {block_application_data_size}\n\
            block local IOs offset: {block_local_ios_offset}\n\
            block local IOs size: {block_local_ios_size}"
            )
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_udp_datagram_header;

    fn create_valid_test_data() -> [u8; 24] {
        [
            'd'.try_into().unwrap(),
            'a'.try_into().unwrap(),
            't'.try_into().unwrap(),
            'a'.try_into().unwrap(),
            'p'.try_into().unwrap(),
            'r'.try_into().unwrap(),
            0x12,
            0x34,
            0x67,
            0x45,
            0x23,
            0x01,
            0xEF,
            0xCD,
            0xAB,
            0x89,
            0x69,
            0xEE,
            0xFF,
            0xC0,
            0x00,
            0x00,
            0x00,
            0x00,
        ]
    }

    #[test]
    fn parse_datagram_marker_from_valid_udp_header() {
        let test_data = create_valid_test_data();
        let result = parse_udp_datagram_header(&test_data);
        assert_eq!(result.datagram_marker, "data");
    }
    #[test]
    fn parse_protocol_from_valid_udp_header() {
        let test_data = create_valid_test_data();
        let result = parse_udp_datagram_header(&test_data);
        assert_eq!(result.protocol, "pr");
    }
    #[test]
    fn parse_version_from_valid_udp_header() {
        let test_data = create_valid_test_data();
        let result = parse_udp_datagram_header(&test_data);
        assert_eq!(result.version_maj, 0x12);
        assert_eq!(result.version_min, 0x34);
    }
    #[test]
    fn parse_total_length_from_valid_udp_header() {
        let test_data = create_valid_test_data();
        let result = parse_udp_datagram_header(&test_data);
        assert_eq!(result.total_length, 0x01234567);
    }
    #[test]
    fn parse_identification_from_valid_udp_header() {
        let test_data = create_valid_test_data();
        let result = parse_udp_datagram_header(&test_data);
        assert_eq!(result.identification, 0x89ABCDEF);
    }
    #[test]
    fn parse_fragment_offset_from_valid_udp_header() {
        let test_data = create_valid_test_data();
        let result = parse_udp_datagram_header(&test_data);
        assert_eq!(result.fragment_offset, 0xC0FFEE69);
    }
}
