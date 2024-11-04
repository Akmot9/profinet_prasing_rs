use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use profinet_rt::ProfinetPacket;
use std::convert::TryFrom;

use pnet::packet::Packet;

/// Example function to handle Ethernet packets and parse Profinet packets.
fn handle_ethernet_packet(ethernet_packet: &EthernetPacket) {
    match ethernet_packet.get_ethertype().0 {
        0x8892 => { // EtherType for Profinet
            match ProfinetPacket::try_from(ethernet_packet.packet()) {
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
    // List all available network interfaces
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.is_up() && !iface.ips.is_empty() && !iface.is_loopback())
        .expect("No suitable network interface found");

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
                let ethernet_packet = EthernetPacket::new(packet).unwrap();
                handle_ethernet_packet(&ethernet_packet);
            },
            Err(e) => eprintln!("Failed to receive packet: {}", e),
        }
    }
}
