extern crate hex;

fn main() {
    // Sample PROFINET packet as a hex string
    let packet_data: Vec<&str> = vec![
        "010ecf000000000e8cd3e37f8892fefe0500030004880001000c02020007706e2d696f2d3200cccc00000000000000000000000000000000",
        "010ecf000000000e8cd2c37b8892fefe0500030001220001000e020200097363616c616e6365680069656d656e732c2053494d4100000000",
        // Add other packets here
    ];

    for data in packet_data {
        let bytes = hex::decode(data).expect("Failed to decode hex string");
        parse_profinet_packet(&bytes);
    }
}

fn parse_profinet_packet(packet: &[u8]) {
    if packet.len() < 14 {
        eprintln!("Packet too short to be valid");
        return;
    }

    // Ethernet II header
    let dest_mac = &packet[0..6];
    let src_mac = &packet[6..12];
    let ether_type = u16::from_be_bytes([packet[12], packet[13]]);

    println!("Destination MAC: {:02x?}", dest_mac);
    println!("Source MAC: {:02x?}", src_mac);
    println!("EtherType: {:#06x}", ether_type);

    if ether_type == 0x8892 {
        // PROFINET Real-Time Protocol
        parse_profinet_realtime(&packet[14..]);
    } else {
        println!("Not a PROFINET packet");
    }
}

fn parse_profinet_realtime(packet: &[u8]) {
    if packet.len() < 16 {
        eprintln!("Packet too short to be a valid PROFINET Real-Time packet");
        return;
    }

    let frame_id = u16::from_be_bytes([packet[0], packet[1]]);
    println!("FrameID: {:#06x}", frame_id);

    if frame_id == 0xfefe {
        parse_profinet_dcp(&packet[2..]);
    } else {
        println!("Not a DCP Identify Request");
    }
}

fn parse_profinet_dcp(packet: &[u8]) {
    if packet.len() < 12 {
        eprintln!("Packet too short to be a valid PROFINET DCP packet");
        return;
    }

    let service_id = packet[0];
    let service_type = packet[1];
    let xid = u32::from_be_bytes([packet[2], packet[3], packet[4], packet[5]]);
    let response_delay = u16::from_be_bytes([packet[6], packet[7]]);
    let dcp_data_length = u16::from_be_bytes([packet[8], packet[9]]);

    println!("ServiceID: {}", service_id);
    println!("ServiceType: {}", service_type);
    println!("XID: {:#010x}", xid);
    println!("ResponseDelay: {}", response_delay);
    println!("DCP Data Length: {}", dcp_data_length);

    // Parsing the block
    let block = &packet[10..];
    if block.len() >= 4 {
        let option = block[0];
        let suboption = block[1];
        let dcp_block_length = u16::from_be_bytes([block[2], block[3]]);

        println!("Option: {}", option);
        println!("Suboption: {}", suboption);
        println!("DCP Block Length: {}", dcp_block_length);

        if block.len() >= (4 + dcp_block_length as usize) {
            let name_of_station = String::from_utf8_lossy(&block[4..4 + dcp_block_length as usize]);
            println!("NameOfStation: {}", name_of_station);
        }
    }
}
