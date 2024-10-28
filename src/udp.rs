#[derive(Debug, PartialEq)]
pub struct UDPDatagramHeader {
    pub datagram_marker: String,
    pub protocol: String,
    pub version_maj: u8,
    pub version_min: u8,
    pub total_length: u32,
    pub identification: u32,
    pub fragment_offset: u32,
}

impl UDPDatagramHeader {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            datagram_marker: String::from_utf8(bytes[0..4].into()).unwrap(),
            protocol: String::from_utf8(bytes[4..6].into()).unwrap(),
            version_maj: bytes[6],
            version_min: bytes[7],
            total_length: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            identification: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            fragment_offset: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
        }
    }
}

#[cfg(test)]
mod udp_datagram_header_tests {
    use array_concat::concat_arrays;

    use crate::udp::UDPDatagramHeader;

    #[test]
    fn parse_from_valid_udp_header() {
        let (test_data, expected_header) = create_valid_test_data();
        let result = UDPDatagramHeader::from_bytes(&test_data);
        assert_eq!(result, expected_header);
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
