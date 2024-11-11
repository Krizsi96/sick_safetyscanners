use bitflags::bitflags;

pub fn bit_status_in_bytes(index: usize, bytes: &[u8]) -> bool {
    const BIT_FLAGS: [BitFlags; 8] = [
        BitFlags::BIT_0,
        BitFlags::BIT_1,
        BitFlags::BIT_2,
        BitFlags::BIT_3,
        BitFlags::BIT_4,
        BitFlags::BIT_5,
        BitFlags::BIT_6,
        BitFlags::BIT_7,
    ];

    let bytes = bytes
        .iter()
        .map(|byte| BitFlags::from_bits_truncate(*byte))
        .collect::<Vec<BitFlags>>();

    let byte = index / u8::BITS as usize;
    let bit = index % u8::BITS as usize;

    bytes[byte].contains(BIT_FLAGS[bit].clone())
}

bitflags! {
    #[derive(Clone)]
    struct BitFlags: u8 {
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
