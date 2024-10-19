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
