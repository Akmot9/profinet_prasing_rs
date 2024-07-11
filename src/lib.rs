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
    pub user_data: Vec<u8>,
    pub cycle_counter: u16,
    pub data_status: DataStatus,
    pub transfer_status: TransferStatus,
}

impl ProfinetPacket {
    pub fn new(data: &[u8]) -> Option<ProfinetPacket> {
        println!("Received data: {:02X?}", data);
        if data.len() < 10 {
            println!("Data too short to be a valid Profinet packet.");
            return None;
        }

        let frame_id_value = u16::from_be_bytes([data[0], data[1]]);
        println!("Frame ID value: {:04x}", frame_id_value);
        let frame_id = FrameId::from_u16(frame_id_value)?;
        println!("Parsed Frame ID: {:?}", frame_id); // Debug trace
        let cycle_counter = u16::from_be_bytes([data[2], data[3]]);
        println!("Cycle Counter: {}", cycle_counter);
        let data_status = match data[4] {
            0x80 => DataStatus::Good,
            0x00 => DataStatus::Bad,
            _ => {
                println!("Invalid data status: {:02x}", data[4]);
                return None;
            }
        };
        println!("Data Status: {:?}", data_status);
        let transfer_status = match data[5] {
            0x00 => TransferStatus::Stopped,
            0x01 => TransferStatus::Running,
            value => {
                println!("Invalid transfer status value: {:02x}", value); // Debug trace
                return None;
            }
        };
        println!("Transfer Status: {:?}", transfer_status); // Debug trace

        let user_data = data[6..].to_vec();
        println!("User Data: {:?}", user_data); // Debug trace

        Some(ProfinetPacket {
            frame_id,
            user_data,
            cycle_counter,
            data_status,
            transfer_status,
        })
    }
}

impl fmt::Display for ProfinetPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Frame ID: {:?}", self.frame_id)?;
        writeln!(f, "User Data: {:?}", self.user_data)?;
        writeln!(f, "Cycle Counter: {}", self.cycle_counter)?;
        writeln!(f, "Data Status: {:?}", self.data_status)?;
        writeln!(f, "Transfer Status: {:?}", self.transfer_status)?;
        Ok(())
    }
}
