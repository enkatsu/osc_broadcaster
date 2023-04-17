extern crate rosc;

use rosc::{OscPacket, encoder};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, UdpSocket};
use std::str::FromStr;

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 32000;
const DEFAULT_SEND_PORT: u16 = 12000;

struct BroadCaster {
    send_addresses: Vec<SocketAddrV4>,
    send_port: u16,
    socket: Option<UdpSocket>,
    listen_ip_address: String,
    listen_port: u16,
}

impl Default for BroadCaster {
    fn default() -> Self {
        BroadCaster {
            send_addresses: Vec::new(),
            send_port: DEFAULT_SEND_PORT,
            socket: None,
            listen_ip_address: DEFAULT_IP_ADDRESS.to_string(),
            listen_port: DEFAULT_PORT,
        }
    }
}

impl BroadCaster {
    fn new() -> Self {
        return Self {
            ..Default::default()
        }
    }

    fn set_listen_ip_address(&mut self, listen_ip_address: String) {
        self.listen_ip_address = listen_ip_address;
    }

    fn set_listen_port(&mut self, listen_port: u16) {
        self.listen_port = listen_port;
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

        let listen_address = SocketAddrV4::new(
            Ipv4Addr::from_str(&self.listen_ip_address).unwrap(),
            self.listen_port
        );
        self.socket = Some(UdpSocket::bind(listen_address).unwrap());
        println!("broadcast server: {}:{}", self.listen_ip_address, self.listen_port);
        println!("send port: {}", self.send_port);

        let mut buf = [0u8; rosc::decoder::MTU];
        loop {
            match self.socket.as_ref().unwrap().recv_from(&mut buf) {
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
                        self.socket.as_ref().unwrap().send_to(&msg_buf, address).unwrap();
                    }
                    println!("OSC address: {}", msg.addr);
                    println!("OSC arguments: {:?}", msg.args);
                }
            }
            OscPacket::Bundle(bundle) => {
                for packet in &bundle.content {
                    let msg_buf = encoder::encode(&packet).unwrap();
                    for address in &self.send_addresses {
                        self.socket.as_ref().unwrap().send_to(&msg_buf, address).unwrap();
                    }
                }
                println!("OSC Bundle: {:?}", bundle);
            }
        }
    }
}

fn get_address_from_arg(arg: &str) -> SocketAddrV4 {
    SocketAddrV4::from_str(arg).unwrap()
}

fn main() {
    let mut broad_caster = BroadCaster::new();
    broad_caster.set_listen_ip_address("127.0.0.1".to_string());
    broad_caster.set_listen_port(32000);
    broad_caster.start();
}
