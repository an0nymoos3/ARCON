use crate::message::{Packet, PacketType};
use anyhow::{anyhow, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{timeout, Duration},
};

/// Represents a connection to a server. Makes it easier to send and recieve requests.
pub struct Client {
    conn: TcpStream,
    timeout: Duration,
}

impl Client {
    /// Creates new Connection.
    pub async fn new(ip: &str, port: &str) -> Result<Self> {
        match TcpStream::connect(format!("{ip}:{port}")).await {
            Ok(stream) => Ok(Self {
                conn: stream,
                timeout: Duration::from_secs(60), // Set defualt timeout to 1 min
            }),
            Err(e) => Err(anyhow!(e)),
        }
    }

    /// Authenticate Client against server.
    pub async fn authenticate(&mut self, server_password: &str) -> Result<String> {
        let packet: Packet = Packet::new(PacketType::Auth, server_password)?;
        let response: Packet = timeout(self.timeout, self.send(packet)).await??;
        Ok(response.body)
    }

    /// Disconnect from RCON server.
    pub async fn disconnect(&mut self) -> Result<()> {
        self.conn.shutdown().await?;
        Ok(())
    }

    /// Lets user manually set time before request timeout.
    pub fn set_timout(&mut self, timeout: u64) {
        let new_timeout: Duration = Duration::from_secs(timeout);
        self.timeout = new_timeout;
    }

    /// Public facing method for easier sending and receiving of commands, with just strings.
    pub async fn send_command(&mut self, command: &str) -> Result<String> {
        let packet: Packet = Packet::new(PacketType::Execcommand, command)?;
        let response: Packet = timeout(self.timeout, self.send(packet)).await??;
        Ok(response.body)
    }

    /// Send message to RCON server.
    async fn send(&mut self, send_packet: Packet) -> Result<Packet> {
        // Convert to bytes and send over tcp
        let packet_bytes = send_packet.encode();
        self.conn.write_all(&packet_bytes).await?;

        // Buffer for receiving response.
        let mut response: Vec<u8> = Vec::new();

        // For some reason clippy wants me to explicitly check for error instead of propagating.
        if let Err(e) = self.conn.read(&mut response).await {
            return Err(anyhow!(e));
        }
        let recv_packet: Packet = Packet::decode(response)?;

        // Make sure server responds to correct packet.
        if send_packet.id != recv_packet.id {
            return Err(anyhow!("Server returned an invalid id!"));
        }

        Ok(recv_packet)
    }
}

// Using [tokio::main] to allow drop() to be asynchronous.
/*impl Drop for Client {
    #[tokio::main]
    async fn drop(&mut self) {
        let _ = self.disconnect().await; // TODO: Maybe look into somehow signalling if disconnect
                                         // failed.
    }
}*/
