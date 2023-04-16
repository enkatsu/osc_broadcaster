extern crate rosc;

use rosc::{OscPacket, encoder};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, UdpSocket};
use std::str::FromStr;

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: i32 = 32000;


struct BroadCaster {
    send_addresses: Vec<SocketAddrV4>,
    send_port: i32,
    socket: UdpSocket,
    listen_ip_address: String,
    listen_port: i32,
}

impl Default for BroadCaster {
    fn default() -> Self {
        let listen_ip_address = "127.0.0.1";
        let listen_port = 32000;
        let listen_address = SocketAddrV4::from_str(
            &format!("{}:{}", listen_ip_address, listen_port).to_string()
        ).unwrap();
        BroadCaster {
            send_addresses: Vec::new(),
            send_port: 12000,
            socket: UdpSocket::bind(listen_address).unwrap(),
            listen_ip_address: listen_ip_address.to_string(),
            listen_port,
        }
    }
}

impl BroadCaster {
    fn new() -> Self {
        return Self {
            ..Default::default()
        }
    }

    fn push_send_address(&mut self, ip_address: String) -> bool {
        let address_str = &format!("{}:{}", ip_address, 12000);
        let address = SocketAddrV4::from_str(address_str).unwrap();
        let found = self.send_addresses.iter().find(|&address| address.ip().to_string() == ip_address);
        if found == None {
            self.send_addresses.push(address);
            return true;
        }
        return false;
    }

    fn remove_send_address(&mut self, ip_address: String) -> bool {
        self.send_addresses.retain(|&send_address| send_address.ip().to_string() != ip_address);
        return true;
    }

    fn print_send_addresses(&self) {
        let send_addresses_string: Vec<String> = self.send_addresses.iter()
            .map(|address| address.to_string())
            .collect();
        let joined = send_addresses_string.join(", ");
        println!("connected: [ {} ]", joined);
    }

    fn start(&mut self) {
        let listen_address = SocketAddrV4::from_str(
            &format!("{}:{}", self.listen_ip_address, self.listen_port).to_string()
        ).unwrap();

        println!("broadcast server: {}", self.socket.local_addr().unwrap().to_string());
        println!("send port: {}", self.send_port);

        let mut buf = [0u8; rosc::decoder::MTU];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((size, address)) => {
                    println!("Received packet with size {} from: {}", size, address);
                    let (_, packet) = rosc::decoder::decode_udp(&buf[..size]).unwrap();
                    self.handle_packet(address.ip(), &packet);
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                    break;
                }
            }
        }
    }

    fn handle_packet(&mut self, ip_addr: IpAddr, packet: &OscPacket) {
        match packet {
            OscPacket::Message(msg) => {
                if msg.addr == "/server/connect" {
                    let connected = self.push_send_address(ip_addr.to_string());
                    if connected {
                        println!("*** connected ***");
                        self.print_send_addresses();
                    }
                } else if msg.addr == "/server/disconnect" {
                    let disconnected = self.remove_send_address(ip_addr.to_string());
                    if disconnected {
                        println!("*** disconnected ***");
                        self.print_send_addresses();
                    }
                } else {
                    let msg_buf = encoder::encode(&packet).unwrap();
                    for address in &self.send_addresses {
                        self.socket.send_to(&msg_buf, address).unwrap();
                    }
                    println!("OSC address: {}", msg.addr);
                    println!("OSC arguments: {:?}", msg.args);
                }
            }
            OscPacket::Bundle(bundle) => {
                for packet in &bundle.content {
                    let msg_buf = encoder::encode(&packet).unwrap();
                    for address in &self.send_addresses {
                        self.socket.send_to(&msg_buf, address).unwrap();
                    }
                }
                println!("OSC Bundle: {:?}", bundle);
            }
        }
    }
}

fn main() {
    let mut broad_caster = BroadCaster::new();
    broad_caster.start();
}
