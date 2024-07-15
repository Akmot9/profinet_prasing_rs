use serde::Serialize;
use std::fmt;

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

impl FrameId {
    fn from_u16(value: u16) -> Option<FrameId> {
        match value {
            0xC000..=0xF7FF => Some(FrameId::Unicast),
            0xF800..=0xFBFF => Some(FrameId::Multicast),
            0xFEFD => Some(FrameId::GetReqSetReqGetRespSetResp),
            0xFEFE => Some(FrameId::IdentifyReq),
            0xFEFF => Some(FrameId::IdentifyResp),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash, Default)]
pub enum DataStatus {
    #[default]
    Good = 0x80,
    Bad = 0x00,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash, Default)]
pub enum TransferStatus {
    #[default]
    Stopped = 0x00,
    Running = 0x01,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq, Hash)]
pub struct ProfinetPacket {
    pub frame_id: FrameId,
    pub service_id: u8,
    pub service_type: u8,
    pub xid: u32,
    pub response_delay: u16,
    pub dcp_data_length: u16,
    pub option: u8,
    pub suboption: u8,
    pub dcp_block_length: u16,
    pub name_of_station: String,
}

impl ProfinetPacket {
    pub fn new(data: &[u8]) -> Option<ProfinetPacket> {
        println!("Received data: {:02X?}", data);
        if data.len() < 16 {
            println!("Data too short to be a valid Profinet packet.");
            return None;
        }

        let frame_id_value = u16::from_be_bytes([data[0], data[1]]);
        println!("Frame ID value: {:04x}", frame_id_value);
        let frame_id = FrameId::from_u16(frame_id_value)?;
        println!("Parsed Frame ID: {:?}", frame_id);

        if frame_id == FrameId::IdentifyReq {
            let service_id = data[2];
            let service_type = data[3];
            let xid = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
            let response_delay = u16::from_be_bytes([data[8], data[9]]);
            let dcp_data_length = u16::from_be_bytes([data[10], data[11]]);

            println!("Service ID: {}", service_id);
            println!("Service Type: {}", service_type);
            println!("XID: {:#010x}", xid);
            println!("Response Delay: {}", response_delay);
            println!("DCP Data Length: {}", dcp_data_length);

            let block = &data[12..];
            if block.len() >= 4 {
                let option = block[0];
                let suboption = block[1];
                let dcp_block_length = u16::from_be_bytes([block[2], block[3]]);

                println!("Option: {}", option);
                println!("Suboption: {}", suboption);
                println!("DCP Block Length: {}", dcp_block_length);

                if block.len() >= (4 + dcp_block_length as usize) {
                    let name_of_station = String::from_utf8_lossy(&block[4..4 + dcp_block_length as usize]).to_string();
                    println!("Name Of Station: {}", name_of_station);

                    return Some(ProfinetPacket {
                        frame_id,
                        service_id,
                        service_type,
                        xid,
                        response_delay,
                        dcp_data_length,
                        option,
                        suboption,
                        dcp_block_length,
                        name_of_station,
                    });
                }
            }
        }

        println!("Not a DCP Identify Request or insufficient data.");
        None
    }
}

impl fmt::Display for ProfinetPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Frame ID: {:?}", self.frame_id)?;
        writeln!(f, "Service ID: {}", self.service_id)?;
        writeln!(f, "Service Type: {}", self.service_type)?;
        writeln!(f, "XID: {:#010x}", self.xid)?;
        writeln!(f, "Response Delay: {}", self.response_delay)?;
        writeln!(f, "DCP Data Length: {}", self.dcp_data_length)?;
        writeln!(f, "Option: {}", self.option)?;
        writeln!(f, "Suboption: {}", self.suboption)?;
        writeln!(f, "DCP Block Length: {}", self.dcp_block_length)?;
        writeln!(f, "Name Of Station: {}", self.name_of_station)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_id_from_u16() {
        assert_eq!(FrameId::from_u16(0xC000), Some(FrameId::Unicast));
        assert_eq!(FrameId::from_u16(0xF800), Some(FrameId::Multicast));
        assert_eq!(FrameId::from_u16(0xFEFD), Some(FrameId::GetReqSetReqGetRespSetResp));
        assert_eq!(FrameId::from_u16(0xFEFE), Some(FrameId::IdentifyReq));
        assert_eq!(FrameId::from_u16(0xFEFF), Some(FrameId::IdentifyResp));
        assert_eq!(FrameId::from_u16(0x0000), None);
    }

    #[test]
    fn test_profinet_packet_new() {
        let data: Vec<u8> = vec![
            0xFE, 0xFE, // Frame ID
            0x01, 0x02, // Service ID and Service Type
            0x00, 0x00, 0x00, 0x01, // XID
            0x00, 0x10, // Response Delay
            0x00, 0x0C, // DCP Data Length
            0x02, // Option
            0x03, // Suboption
            0x00, 0x04, // DCP Block Length
            b'T', b'E', b'S', b'T' // Name Of Station
        ];

        let packet = ProfinetPacket::new(&data).expect("Failed to parse packet");

        assert_eq!(packet.frame_id, FrameId::IdentifyReq);
        assert_eq!(packet.service_id, 0x01);
        assert_eq!(packet.service_type, 0x02);
        assert_eq!(packet.xid, 0x00000001);
        assert_eq!(packet.response_delay, 0x0010);
        assert_eq!(packet.dcp_data_length, 0x000C);
        assert_eq!(packet.option, 0x02);
        assert_eq!(packet.suboption, 0x03);
        assert_eq!(packet.dcp_block_length, 0x0004);
        assert_eq!(packet.name_of_station, "TEST");
    }

    #[test]
    fn test_profinet_packet_new_with_real_data() {
        let data: Vec<u8> = vec![
            0xFE, 0xFE, // Frame ID
            0x05, 0x00, // Service ID and Service Type
            0x03, 0x00, 0x01, 0x44, // XID
            0x00, 0x01, // Response Delay
            0x00, 0x0E, // DCP Data Length
            0x02, // Option
            0x02, // Suboption
            0x00, 0x09, // DCP Block Length
            b's', b'c', b'a', b'l', b'a', b'n', b'c', b'e', b'h', // Name Of Station
            b'e', b'm', b'e', b'n', b's', b',', b' ', b'S', b'I', b'M', b'A' // Continuation of the Name Of Station
        ];

        let packet = ProfinetPacket::new(&data);

        assert!(packet.is_some());
        let packet = packet.unwrap();
        assert_eq!(packet.frame_id, FrameId::IdentifyReq);
        assert_eq!(packet.service_id, 0x05);
        assert_eq!(packet.service_type, 0x00);
        assert_eq!(packet.xid, 0x03000144);
        assert_eq!(packet.response_delay, 0x0001);
        assert_eq!(packet.dcp_data_length, 0x000E);
        assert_eq!(packet.option, 0x02);
        assert_eq!(packet.suboption, 0x02);
        assert_eq!(packet.dcp_block_length, 0x0009);
        assert_eq!(packet.name_of_station, "scalanceh");
    }
}
