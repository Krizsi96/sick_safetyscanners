extern crate sick_safetyscanners;

use sick_safetyscanners::data_output::{DataOutputHeader, OutputConfigurationBlock};
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
        println!("\n{:?}", datagram_header);

        let data_field = &buf[24..message_size];

        if datagram_header.fragment_offset == 0 {
            let data_output_header = DataOutputHeader::from_bytes(&data_field);
            println!("{:?}", data_output_header);

            let start_idx: usize = data_output_header.output_configuration_block.offset as usize;
            let end_idx: usize =
                start_idx + (data_output_header.output_configuration_block.size as usize);
            let output_configuration = &data_field[start_idx..end_idx];
            let output_configuration = OutputConfigurationBlock::from_bytes(output_configuration);
            println!("\n{:?}", output_configuration);
        } else {
            println!(
                "data field: {:?}",
                data_field
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<Vec<String>>()
            );
        }
    }
    Ok(())
}
