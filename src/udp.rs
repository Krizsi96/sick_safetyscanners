use std::ops::Range;

const DATAGRAM_MARKER_BYTES: Range<usize> = 0..4;
const PROTOCOL_BYTES: Range<usize> = 4..6;
const VERSION_MAJ_BYTE: usize = 6;
const VERSION_MIN_BYTE: usize = 7;
const TOTAL_LENGTH_BYTES: Range<usize> = 8..12;
const IDENTIFICATION_BYTES: Range<usize> = 12..16;
const FRAGMENT_OFFSET_BYTES: Range<usize> = 16..20;

#[derive(Debug)]
pub struct UDPDatagramHeader {
    datagram_marker: String,
    protocol: String,
    version_maj: u8,
    version_min: u8,
    total_length: u32,
    identification: u32,
    pub fragment_offset: u32,
}

impl UDPDatagramHeader {
    pub fn from_bytes(buffer: &[u8]) -> UDPDatagramHeader {
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
}
#[cfg(test)]
mod tests {
    use array_concat::concat_arrays;

    use crate::udp::UDPDatagramHeader;

    #[test]
    fn parse_datagram_marker_from_valid_udp_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = UDPDatagramHeader::from_bytes(&test_data);
        assert_eq!(result.datagram_marker, expected_header.datagram_marker);
    }

    #[test]
    fn parse_protocol_from_valid_udp_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = UDPDatagramHeader::from_bytes(&test_data);
        assert_eq!(result.protocol, expected_header.protocol);
    }

    #[test]
    fn parse_version_from_valid_udp_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = UDPDatagramHeader::from_bytes(&test_data);
        assert_eq!(result.version_maj, expected_header.version_maj);
        assert_eq!(result.version_min, expected_header.version_min);
    }

    #[test]
    fn parse_total_length_from_valid_udp_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = UDPDatagramHeader::from_bytes(&test_data);
        assert_eq!(result.total_length, expected_header.total_length);
    }

    #[test]
    fn parse_identification_from_valid_udp_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = UDPDatagramHeader::from_bytes(&test_data);
        assert_eq!(result.identification, expected_header.identification);
    }

    #[test]
    fn parse_fragment_offset_from_valid_udp_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = UDPDatagramHeader::from_bytes(&test_data);
        assert_eq!(result.fragment_offset, expected_header.fragment_offset);
    }

    fn create_valid_test_data() -> ([u8; 24], UDPDatagramHeader) {
        let datagram_marker: [u8; 4] = ['d', 'a', 't', 'a']
            .iter()
            .map(|&c| c as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        let protocol: [u8; 2] = ['p', 'r']
            .iter()
            .map(|&char| char as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        let version_maj = [0x12];
        let version_min = [0x34];
        let total_length = [0x67, 0x45, 0x23, 0x01];
        let identification = [0xEF, 0xCD, 0xAB, 0x89];
        let fragment_offset = [0x69, 0xEE, 0xFF, 0xC0];

        (
            concat_arrays!(
                datagram_marker,
                protocol,
                version_maj,
                version_min,
                total_length,
                identification,
                fragment_offset,
                [0u8; 4]
            ),
            UDPDatagramHeader {
                datagram_marker: String::from_utf8(datagram_marker.into()).unwrap(),
                protocol: String::from_utf8(protocol.into()).unwrap(),
                version_maj: u8::from_le_bytes(version_maj),
                version_min: u8::from_le_bytes(version_min),
                total_length: u32::from_le_bytes(total_length),
                identification: u32::from_le_bytes(identification),
                fragment_offset: u32::from_le_bytes(fragment_offset),
            },
        )
    }
}
