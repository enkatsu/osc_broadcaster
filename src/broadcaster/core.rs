use super::BroadCaster;
use rosc::{OscPacket, encoder, OscBundle, OscMessage};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, UdpSocket};
use std::str::FromStr;

const DEFAULT_IP_ADDRESS: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 32000;
const DEFAULT_SEND_PORT: u16 = 12000;

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

    pub fn start(&mut self) {
        BroadCaster::print_settings(&self.listen_ip_address, &self.listen_port, &self.send_port);

        let listen_address = SocketAddrV4::new(
            Ipv4Addr::from_str(&self.listen_ip_address).unwrap(),
            self.listen_port
        );
        self.socket = Some(UdpSocket::bind(listen_address).unwrap());

        let mut buf = [0u8; rosc::decoder::MTU];
        loop {
            match self.socket.as_ref().unwrap().recv_from(&mut buf) {
                Ok((size, address)) => {
                    let (_, packet) = rosc::decoder::decode_udp(&buf[..size])
                        .unwrap();
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
                println!("Broadcast bundle");
                self.send_bundle(bundle);
                println!("OSC Bundle: {:?}", bundle);
            }
        }
    }

    fn handle_message(&mut self, message: &OscMessage, ip_address: IpAddr, packet: &OscPacket) {
        match &message.addr[..] {
            "/server/connect" => {
                if self.push_send_address(ip_address.to_string()) {
                    println!("*** Connected ***");
                    BroadCaster::print_send_addresses(&self.send_addresses);
                }
            },
            "/server/disconnect" => {
                if self.remove_send_address(ip_address.to_string()) {
                    println!("*** Disconnected ***");
                    BroadCaster::print_send_addresses(&self.send_addresses);
                }
            },
            _ => {
                if self.send_message(packet) > 0 {
                    println!("*** Broadcast message ***");
                    BroadCaster::print_message(message);
                }
            }
        }
    }

    fn send_message(&self, packet: &OscPacket) -> usize {
        let msg_buf = encoder::encode(&packet).unwrap();
        for address in &self.send_addresses {
            self.socket.as_ref().unwrap().send_to(&msg_buf, address).unwrap();
        }
        self.send_addresses.len()
    }

    fn send_bundle(&self, bundle: &OscBundle) -> usize {
        for address in &self.send_addresses {
            for packet in &bundle.content {
                let msg_buf = encoder::encode(&packet).unwrap();
                self.socket.as_ref().unwrap().send_to(&msg_buf, address).unwrap();
            }
        }
        self.send_addresses.len()
    }

    fn push_send_address(&mut self, ip_address: String) -> bool {
        let address_str = &format!("{}:{}", ip_address, 12000);
        let address = SocketAddrV4::from_str(address_str).unwrap();
        let found = self.send_addresses.iter()
            .find(|&address| address.ip().to_string() == ip_address);
        if found == None {
            self.send_addresses.push(address);
            return true;
        }
        return false;
    }

    fn remove_send_address(&mut self, ip_address: String) -> bool {
        self.send_addresses
            .retain(|&send_address| send_address.ip().to_string() != ip_address);
        return true;
    }
}
