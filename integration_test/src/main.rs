use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::EthernetPacket;
use profinet_rt::ProfinetPacket;
use std::convert::TryFrom;
use std::env;
use pnet::packet::Packet;

/// Example function to handle Ethernet packets and parse Profinet packets.
fn handle_ethernet_packet(ethernet_packet: &EthernetPacket) {
    match ethernet_packet.get_ethertype().0 {
        0x8892 => { // EtherType for Profinet
            println!("Received Profinet packet: {:?}", ethernet_packet.payload());
            match ProfinetPacket::try_from(ethernet_packet.payload()) {
                Ok(profinet_packet) => {
                    println!("Parsed Profinet Packet: {:?}", profinet_packet);
                },
                Err(err) => {
                    println!("Failed to parse Profinet packet: {}", err);
                    println!("Packet data: {:?}", ethernet_packet.payload());
                }
            }
        },
        _ => println!("Non-Profinet packet."),
    }
}

fn main() {
    // Check for interface name as a command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <interface_name>", args[0]);
        std::process::exit(1);
    }

    let interface_name = &args[1];
    
    // List all available network interfaces
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("No matching network interface found");

    println!("Using interface: {}", interface.name);

    // Create a channel to receive packets from the network interface
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel: {}", e),
    };

    println!("Listening for Ethernet packets...");

    // Continuously listen for packets and parse them
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    handle_ethernet_packet(&ethernet_packet);
                } else {
                    eprintln!("Failed to parse Ethernet packet.");
                }
            },
            Err(e) => eprintln!("Failed to receive packet: {}", e),
        }
    }
}
