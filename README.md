# Profinet Packet Handler

This Rust crate provides functionality to detect and parse payloads from Ethernet packets into Profinet packets. It supports different frame IDs, data statuses, and transfer statuses, as defined in the Profinet protocol.

## Features

- Parse raw Ethernet payloads into structured Profinet packets.
- Supports various frame IDs, data statuses, and transfer statuses.
- Automatically derive default values for enums where applicable.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
profinet_rt = "0.1.0"
```

## Usage

Here's an example of how to use the crate to parse a Profinet packet from Ethernet payload:

```rust
use profinet_rt::{ProfinetPacket, EtherTypes};

// Example function to handle Ethernet packets
fn handle_ethernet_packet(ethernet_packet: &EthernetPacket) {
    match ethernet_packet.get_ethertype() {
        EtherTypes::Profinet => {
            if let Some(profinet_packet) = ProfinetHandler::get_layer_3(ethernet_packet.payload()) {
                println!("Parsed Profinet Packet: {}", profinet_packet);
            } else {
                println!("Failed to parse Profinet packet.");
            }
        },
        _ => println!("Non-Profinet packet."),
    }
}

fn main() {
    // Assuming `ethernet_packet` is obtained from some packet capturing library
    let ethernet_packet = ...;
    handle_ethernet_packet(&ethernet_packet);
}
```

## Profinet Packet Structure

The `ProfinetPacket` struct consists of the following fields:

- `frame_id`: The frame ID of the packet, represented by the `FrameId` enum.
- `user_data`: A vector of bytes representing the user data of the packet.
- `cycle_counter`: A 16-bit unsigned integer representing the cycle counter.
- `data_status`: The data status of the packet, represented by the `DataStatus` enum.
- `transfer_status`: The transfer status of the packet, represented by the `TransferStatus` enum.

### FrameId Enum

```rust
#[repr(u16)]
#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash, Default)]
pub enum FrameId {
    #[default]
    Unicast = 0xC000,
    Multicast = 0xF800,
    GetReqSetReqGetRespSetResp = 0xFEFD,
    IdentifyReq = 0xFEFE,
    IdentifyResp = 0xFEFF,
}
```

### DataStatus Enum

```rust
#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash, Default)]
pub enum DataStatus {
    #[default]
    Good = 0x80,
    Bad = 0x00,
}
```

### TransferStatus Enum

```rust
#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash, Default)]
pub enum TransferStatus {
    #[default]
    Stopped = 0x00,
    Running = 0x01,
}
```

## Parsing Profinet Packets

To parse a Profinet packet from raw data, use the `ProfinetPacket::new` method:

```rust
let raw_data: &[u8] = &[
    // Example Profinet packet data
    0xC0, 0x00, // Frame ID
    0x00, 0x01, // Cycle Counter
    0x80, // Data Status
    0x01, // Transfer Status
    0xDE, 0xAD, 0xBE, 0xEF // User Data
];

if let Some(packet) = ProfinetPacket::new(raw_data) {
    println!("Parsed Profinet Packet: {}", packet);
} else {
    println!("Failed to parse Profinet packet.");
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

