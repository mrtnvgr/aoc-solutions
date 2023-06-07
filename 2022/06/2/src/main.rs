use std::{collections::VecDeque, fs};

fn main() {
    let buffer = fs::read_to_string("input.txt").unwrap();

    let mut first_packet_offset = 0;
    let mut packet_buffer: VecDeque<char> = VecDeque::new();

    for packet in buffer.trim().chars() {
        first_packet_offset += 1;

        packet_buffer.push_back(packet);

        if packet_buffer.len() == 14 {
            let mut buffer: Vec<char> = packet_buffer.clone().into();
            buffer.sort_unstable();

            let mut deduped_buffer = buffer.clone();
            deduped_buffer.dedup();

            if buffer == deduped_buffer {
                break;
            }

            packet_buffer.pop_front();
        }
    }

    println!("{first_packet_offset}");
}
