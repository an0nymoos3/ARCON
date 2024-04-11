use crate::message::{Packet, PacketType};
use anyhow::{anyhow, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

/// Represents a connection to a server. Makes it easier to send and recieve requests.
pub struct Connection {
    conn: TcpStream,
}

impl Connection {
    /// Creates new Connection.
    pub async fn new(ip: &str, port: &str) -> Result<Self> {
        match TcpStream::connect(format!("{ip}:{port}")).await {
            Ok(stream) => Ok(Self { conn: stream }),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// Establish connection to RCON server.
    pub async fn connect(&mut self, server_password: &str) -> Result<()> {
        let packet: Packet = Packet::new(PacketType::Auth, server_password)?;
        self.send(packet).await?;
        Ok(())
    }

    /// Disconnect from RCON server.
    pub async fn disconnect(&mut self) -> Result<()> {
        self.conn.shutdown().await?;
        Ok(())
    }

    /// Send message to RCON server.
    pub async fn send(&mut self, message: Packet) -> Result<Packet> {
        let payload = message.encode();

        self.conn.write_all(&payload).await?;

        let mut response: Vec<u8> = Vec::new();

        // For some reason clippy wants me to explicitly check for error instead of propagating.
        if let Err(e) = self.conn.read(&mut response).await {
            return Err(anyhow!(e));
        }

        Packet::decode(response)
    }
}
