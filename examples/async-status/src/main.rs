use std::net::Ipv4Addr;

use protocol_packets::{
    clientbound::status::StatusResponse,
    protocol_core::{
        data::VarInt,
        packet::Packet,
        rw::{AsyncReadable, AsyncWriteable},
        tokio::io::{AsyncRead, AsyncWrite},
    },
    serverbound::{handshaking::Handshake, status::StatusRequest},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let ipv4: Ipv4Addr = args[1]
        .parse()
        .expect("please input a valid ipv4 as the first input");
    let port: u16 = args[2]
        .parse()
        .expect("please input a valid port as the second input");

    let mut stream = TcpStream::connect((ipv4, port)).await.unwrap();

    write_packet(
        Handshake {
            protocol_version: 762.into(),
            server_address: ipv4.to_string(),
            server_port: port,
            next_state: 1.into(),
        },
        &mut stream,
    )
    .await;

    write_packet(StatusRequest {}, &mut stream).await;

    let response = read_packet::<StatusResponse, _>(&mut stream).await;

    dbg!(&response.json_response);
}

async fn write_packet<P, W>(packet: P, mut write: &mut W)
where
    P: Packet + AsyncWriteable,
    W: AsyncWrite + Send + Sync + Unpin,
{
    let mut buf = Vec::<u8>::new();
    VarInt::new(P::ID as i32)
        .write_async(&mut buf)
        .await
        .unwrap();
    packet.write_async(&mut buf).await.unwrap();

    VarInt::new(buf.len() as i32)
        .write_async(&mut write)
        .await
        .unwrap();
    write.write_all(&mut buf).await.unwrap();
}

async fn read_packet<P, R>(mut read: &mut R) -> P
where
    P: Packet + AsyncReadable,
    R: AsyncRead + Send + Sync + Unpin,
{
    let size = *VarInt::read_async(&mut read).await.unwrap();
    let mut buf = vec![0u8; size as usize];
    read.read_exact(&mut buf).await.unwrap();
    let mut buf = &buf[..];
    let _ = VarInt::read_async(&mut buf).await.unwrap();
    return P::read_async(&mut buf).await.unwrap();
}
