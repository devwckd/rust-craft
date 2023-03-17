use std::{
    io::{Read, Write},
    net::{Ipv4Addr, TcpStream},
};

use protocol_packets::{
    clientbound::status::StatusResponse,
    protocol_core::{
        data::VarInt,
        packet::Packet,
        rw::{SyncReadable, SyncWriteable},
    },
    serverbound::{handshaking::Handshake, status::StatusRequest},
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let ipv4: Ipv4Addr = args[1]
        .parse()
        .expect("please input a valid ipv4 as the first input");
    let port: u16 = args[2]
        .parse()
        .expect("please input a valid port as the second input");

    let mut stream = TcpStream::connect((ipv4, port)).unwrap();

    write_packet(
        Handshake {
            protocol_version: 762.into(),
            server_address: ipv4.to_string(),
            server_port: port,
            next_state: 1.into(),
        },
        &mut stream,
    );

    write_packet(StatusRequest {}, &mut stream);

    let response = read_packet::<StatusResponse, _>(&mut stream);

    dbg!(&response.json_response);
}

fn write_packet<P, W>(packet: P, mut write: &mut W)
where
    P: Packet + SyncWriteable,
    W: Write,
{
    let mut buf = Vec::<u8>::new();
    VarInt::new(P::ID as i32).write_sync(&mut buf).unwrap();
    packet.write_sync(&mut buf).unwrap();

    VarInt::new(buf.len() as i32)
        .write_sync(&mut write)
        .unwrap();
    write.write_all(&mut buf).unwrap();
}

fn read_packet<P, R>(mut read: &mut R) -> P
where
    P: Packet + SyncReadable,
    R: Read,
{
    let size = *VarInt::read_sync(&mut read).unwrap();
    let mut buf = vec![0u8; size as usize];
    read.read_exact(&mut buf).unwrap();
    let mut buf = &buf[..];
    let _ = VarInt::read_sync(&mut buf).unwrap();
    return P::read_sync(&mut buf).unwrap();
}
