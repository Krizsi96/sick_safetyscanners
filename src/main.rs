use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("192.168.1.15:1025")?;

        // Receives a single datagram message on the socket. If 'buf' is too small
        // to hold the message, it will be cut off.
        let mut buf = [0; 2048];
        let (amt, src) = socket.recv_from(&mut buf)?;

        println!(
            "Received hex: {:?}",
            buf[..amt]
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<Vec<String>>()
        );

        let datagram_marker = std::str::from_utf8(&buf[0..4]).unwrap();
        let protocol = std::str::from_utf8(&buf[4..6]).unwrap();
        let version_maj = buf[6];
        let version_min = buf[7];
        let total_length = &buf[8..12];
        let total_length = u32::from_le_bytes(total_length.try_into().unwrap());
        let identification = &buf[12..16];
        let identification = u32::from_le_bytes(identification.try_into().unwrap());
        let fragment_offset = &buf[16..20];
        let fragment_offset = u32::from_le_bytes(fragment_offset.try_into().unwrap());

        println!(
            "datagram marker: {:?}\n\
            protocol: {:?}\n\
            version: {:?}.{:?}\n\
            total length: {:?}\n\
            identification: {:?}\n\
            fragment offset: {:?}",
            datagram_marker,
            protocol,
            version_maj,
            version_min,
            total_length,
            identification,
            fragment_offset
        );
    }
    Ok(())
}
