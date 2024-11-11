extern crate sick_safetyscanners;

use std::net::UdpSocket;

use sick_safetyscanners::data_output::device_status::DeviceStatus;
use sick_safetyscanners::data_output::field_interruption::FieldInterruptions;
use sick_safetyscanners::data_output::measurement_data::MeasurementDataBlock;
use sick_safetyscanners::data_output::output_configuration::OutputConfigurationBlock;
use sick_safetyscanners::data_output::DataOutputHeader;
use sick_safetyscanners::udp::UDPDatagramHeader;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("192.168.1.15:1025")?;

        // Receives a single datagram message on the socket. If 'buf' is too small
        // to hold the message, it will be cut off.
        let mut buf = [0; 2048];
        let mut udp_message_buffer = [0u8; 4096];
        let mut identification = 0;
        let mut total_length: u32 = 0;
        let mut udp_bytes_counter: u32 = 0;
        let mut program_finished = false;

        while !program_finished {
            let (message_size, source_address) = socket.recv_from(&mut buf)?;
            println!("\n-------\nreceived packet from {}", source_address);

            let datagram_header = UDPDatagramHeader::from_bytes(&buf);
            println!("\n{:?}", datagram_header);

            let data_field = &buf[24..message_size];

            if datagram_header.fragment_offset == 0 {
                udp_message_buffer = [0u8; 4096];
                udp_bytes_counter = 0;
                total_length = 0;
                println!(
                    "{:?}, \nbyte counter: {udp_bytes_counter}\ntotal length: {total_length}",
                    udp_message_buffer
                );

                identification = datagram_header.identification;
                total_length = datagram_header.total_length;
                let start_index: usize = datagram_header.fragment_offset as usize;
                let end_index: usize = start_index + data_field.len();
                println!("slice starts: {start_index}, ends: {end_index}");
                udp_bytes_counter += data_field.len() as u32;
                udp_message_buffer[start_index..end_index].copy_from_slice(data_field);
                println!(
                    "{:?}, \nbyte counter: {udp_bytes_counter}\ntotal length: {total_length}",
                    udp_message_buffer
                );
            } else if datagram_header.identification == identification {
                let start_index: usize = datagram_header.fragment_offset as usize;
                let end_index: usize = start_index + data_field.len();
                println!("slice starts: {start_index}, ends: {end_index}");
                udp_bytes_counter += data_field.len() as u32;
                udp_message_buffer[start_index..end_index].copy_from_slice(data_field);
                println!(
                    "{:?}, \nbyte counter: {udp_bytes_counter}\ntotal length: {total_length}",
                    udp_message_buffer
                );

                if udp_bytes_counter == total_length {
                    let data_output_header = DataOutputHeader::from_bytes(&udp_message_buffer);
                    println!("{:?}", data_output_header);

                    let start_idx: usize = data_output_header.device_status_block.offset as usize;
                    let end_idx: usize =
                        start_idx + data_output_header.device_status_block.size as usize;
                    let device_status = &udp_message_buffer[start_idx..end_idx];
                    let device_status = DeviceStatus::from_bytes(device_status);
                    println!("\n{:?}", device_status);

                    let start_idx: usize =
                        data_output_header.output_configuration_block.offset as usize;
                    let end_idx: usize =
                        start_idx + (data_output_header.output_configuration_block.size as usize);
                    let output_configuration = &udp_message_buffer[start_idx..end_idx];
                    let output_configuration =
                        OutputConfigurationBlock::from_bytes(output_configuration);
                    println!("\n{:?}", output_configuration);

                    let start_idx: usize =
                        data_output_header.measurement_data_block.offset as usize;
                    let end_idx: usize =
                        start_idx + (data_output_header.measurement_data_block.size as usize);
                    let measurement_data = &udp_message_buffer[start_idx..end_idx];
                    let measurement_data = MeasurementDataBlock::from_bytes(measurement_data);
                    println!("\n{:?}", measurement_data);

                    let start_idx: usize =
                        data_output_header.field_interruption_block.offset as usize;
                    let end_idx: usize =
                        start_idx + (data_output_header.field_interruption_block.size as usize);
                    let field_interruptions = &udp_message_buffer[start_idx..end_idx];
                    match FieldInterruptions::from_bytes(field_interruptions) {
                        Some(field_interruptions) => println!("\n{:?}", field_interruptions),
                        None => {
                            println!("\nThere is no field interruption block in the datagram.")
                        }
                    };

                    program_finished = true;
                }
            }
        }
    }
    Ok(())
}
