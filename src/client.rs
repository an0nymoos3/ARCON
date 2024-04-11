use std::sync::atomic;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::message::{AError, AResult, Packet, PacketType};

/// Represents a connection to a server. Makes it easier to send and recieve requests.
struct Connection {
    conn: TcpStream,
    id: atomic::AtomicI32,
}

impl Connection {
    /// Creates new Connection.
    pub async fn new(ip: &str, port: &str) -> Result<Self, String> {
        match TcpStream::connect(format!("{ip}:{port}")).await {
            Ok(stream) => Ok(Self {
                conn: stream,
                id: atomic::AtomicI32::new(0),
            }),
            Err(e) => return Err(e.to_string()),
        }
    }

    /// Establish connection to RCON server.
    pub async fn connect(&self, server_password: &str) -> AResult {
        let packet: Packet = Packet::new(PacketType::Auth, server_password);
        self.send(packet).await
    }

    /// Disconnect from RCON server.
    pub fn disconnect(&self) -> AResult {}

    /// Send message to RCON server.
    pub async fn send(&self, message: Packet) -> AResult {
        let payload = message.encode();

        if let Err(e) = self.conn.write_all(&payload).await {
            return AResult::Err(AError::TcpError(e.to_string()));
        }

        let mut response: Vec<u8> = Vec::new();
    }
}
