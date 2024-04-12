use anyhow::{anyhow, Result};
use rand::{rngs::StdRng, Rng, SeedableRng};

const MAX_SIZE: i32 = 4096;
const PACKET_PADDING: i32 = 10; // See Valve's documentaion under "Packet Size"

/// Packet types as defined by valve.
/// Currently allowing dead_code, might use the remaining PacketTypes in the future.
#[allow(dead_code)]
pub enum PacketType {
    Auth,
    ResponseValue,
    AuthResponse,
    Execcommand,
}

/// Rust representation of a packet.
pub struct Packet {
    size: i32,
    pub id: i32,
    packet_type: i32,
    pub body: String,
}

impl Packet {
    /// Construct a new Packet to send to server.
    pub fn new(p_type: PacketType, body: &str) -> Result<Self> {
        let size: i32 = body.len() as i32 + PACKET_PADDING; // Calculate packet size
        let id: i32 = StdRng::from_entropy().gen(); // Generate a random id

        // Convert p_type to packet type number.
        let packet_type: i32 = match p_type {
            PacketType::Auth => 3,
            PacketType::ResponseValue => 0,
            _ => 2, // Other 2 options both have value 2
        };

        // Make sure request isn't too big.
        if size > MAX_SIZE {
            return Err(anyhow!("Packet size exceeds MAX_SIZE! (4096)"));
        }

        Ok(Self {
            size,
            id,
            packet_type,
            body: body.to_string(),
        })
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
    pub fn decode(bytes: Vec<u8>) -> Result<Packet> {
        let size: i32 = i32::from_le_bytes(bytes[0..4].try_into()?);
        let id: i32 = i32::from_le_bytes(bytes[4..8].try_into()?);
        let packet_type: i32 = i32::from_le_bytes(bytes[8..12].try_into()?);
        let body: String = String::from_utf8(bytes[12..].to_vec())?;

        Ok(Packet {
            size,
            id,
            packet_type,
            body,
        })
    }
}
