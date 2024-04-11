const MAX_SIZE: i32 = 4096;
const PACKET_PADDING: i32 = 10; // See Valve's documentaion under "Packet Size"

pub enum AResult {
    Ok(Packet),
    Err(AError),
}

pub enum AError {
    AuthError(String),
    TcpError(String),
}

/// Packet types as defined by valve.
pub enum PacketType {
    Auth,
    ResponseValue,
    AuthResponse,
    Execcommand,
}

pub struct Packet {
    size: i32,
    id: i32,
    packet_type: i32,
    body: String,
}

impl Packet {
    /// Construct a new Packet to send to server.
    pub fn new(p_type: PacketType, body: &str) -> Self {
        let size: i32 = body.len() as i32 + PACKET_PADDING;
        let id = 1;
        let packet_type: i32 = match p_type {
            PacketType::Auth => 3,
            PacketType::ResponseValue => 0,
            _ => 2, // Other 2 options both have value 2
        };

        Self {
            size,
            id,
            packet_type,
            body: body.to_string(),
        }
    }

    /// Encodes packet to bytes, that RCON expects
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.packet_type.to_le_bytes());
        bytes.extend_from_slice(self.body.as_bytes());
        bytes.extend_from_slice(&[0, 0]); // Add empty terminating string.

        bytes
    }

    /// Basically the reverse of encode. Takes raw data in the form of a vec of bytes, converts it
    /// back into a Packet struct.
    pub fn decode(bytes: Vec<u8>) -> AResult {
        let size: i32 = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let id: i32 = i32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let packet_type: i32 = i32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let body: String = String::from_utf8(bytes[12..].to_vec()).unwrap();

        AResult::Ok(Packet {
            size,
            id,
            packet_type,
            body,
        })
    }
}
