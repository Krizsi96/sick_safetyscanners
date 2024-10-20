extern crate sick_safetyscanners;

use sick_safetyscanners::data_output::DataOutputHeader;
use sick_safetyscanners::udp::parse_udp_datagram_header;
use std::net::UdpSocket;

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
            let data_output_header = DataOutputHeader::from_bytes(&data_field);
            println!("{:?}", data_output_header)
        }
    }
    Ok(())
}
