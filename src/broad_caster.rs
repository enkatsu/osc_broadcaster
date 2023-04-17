extern crate rosc;

use rosc::{OscPacket, encoder, OscBundle, OscMessage};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, UdpSocket};
use std::str::FromStr;


const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 32000;
const DEFAULT_SEND_PORT: u16 = 12000;

pub struct BroadCaster {
    send_addresses: Vec<SocketAddrV4>,
    pub send_port: u16,
    socket: Option<UdpSocket>,
    pub listen_ip_address: String,
    pub listen_port: u16,
}

impl BroadCaster {
    pub fn new() -> Self {
        return Self {
            send_addresses: Vec::new(),
            send_port: DEFAULT_SEND_PORT,
            socket: None,
            listen_ip_address: DEFAULT_IP_ADDRESS.to_string(),
            listen_port: DEFAULT_PORT,
        }
    }

    pub fn set_listen_port(&mut self, listen_port: u16) {
        self.listen_port = listen_port;
    }

    pub fn set_send_port(&mut self, send_port: u16) {
        self.send_port = send_port;
    }

    pub fn start(&mut self) {
        let listen_address = SocketAddrV4::new(
            Ipv4Addr::from_str(&self.listen_ip_address).unwrap(),
            self.listen_port
        );
        self.socket = Some(UdpSocket::bind(listen_address).unwrap());

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

    fn handle_packet(&mut self, ip_address: IpAddr, packet: &OscPacket) {
        match packet {
            OscPacket::Message(message) => {
                self.handle_message(message, ip_address, packet);
            }
            OscPacket::Bundle(bundle) => {
                println!("*** broadcast bundle ***");
                self.send_bundle(bundle);
                println!("OSC Bundle: {:?}", bundle);
            }
        }
    }

    fn handle_message(&mut self, message: &OscMessage, ip_address: IpAddr, packet: &OscPacket) {
        if message.addr == "/server/connect" {
            self.server_connect(ip_address);
        } else if message.addr == "/server/disconnect" {
            self.server_disconnect(ip_address);
        } else {
            println!("*** broadcast message ***");
            self.send_message(packet);
            println!("OSC address: {}", message.addr);
            println!("OSC arguments: {:?}", message.args);
        }
    }

    fn server_connect(&mut self, ip_address: IpAddr) {
        let connected = self.push_send_address(ip_address.to_string());
        if connected {
            println!("*** connected ***");
            self.print_send_addresses();
        }
    }

    fn server_disconnect(&mut self, ip_address: IpAddr) {
        let disconnected = self.remove_send_address(ip_address.to_string());
        if disconnected {
            println!("*** disconnected ***");
            self.print_send_addresses();
        }
    }

    fn send_message(&self, packet: &OscPacket) {
        let msg_buf = encoder::encode(&packet).unwrap();
        for address in &self.send_addresses {
            self.socket.as_ref().unwrap().send_to(&msg_buf, address).unwrap();
        }
    }

    fn send_bundle(&self, bundle: &OscBundle) {
        for address in &self.send_addresses {
            for packet in &bundle.content {
                let msg_buf = encoder::encode(&packet).unwrap();
                self.socket.as_ref().unwrap().send_to(&msg_buf, address).unwrap();
            }
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
}
