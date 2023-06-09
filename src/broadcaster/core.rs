use super::BroadCaster;
use rosc::{OscPacket, encoder, OscBundle, OscMessage, OscType, OscError};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, UdpSocket};
use std::str::FromStr;

const DEFAULT_IP_ADDRESS: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 32000;
const DEFAULT_SEND_PORT: u16 = 12000;

impl Default for BroadCaster {
    fn default() -> Self {
        Self::new()
    }
}

impl BroadCaster {
    pub fn new() -> Self {
        Self {
            send_addresses: Vec::new(),
            send_port: DEFAULT_SEND_PORT,
            socket: None,
            listen_ip_address: DEFAULT_IP_ADDRESS.to_string(),
            listen_port: DEFAULT_PORT,
        }
    }

    pub fn start(&mut self) {
        BroadCaster::print_settings(&self.listen_ip_address, &self.listen_port, &self.send_port);
        if !self.send_addresses.is_empty() {
            BroadCaster::print_send_addresses(&self.send_addresses);
        }

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
                let port: u16 = match self.convert_connection_message_to_port_number(message) {
                    Ok(i) => i,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                if self.push_send_address(ip_address, port) {
                    println!("*** Connected ***");
                    BroadCaster::print_send_addresses(&self.send_addresses);
                }
            },
            "/server/disconnect" => {
                let port: u16 = match self.convert_connection_message_to_port_number(message) {
                    Ok(i) => i,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                if self.remove_send_address(ip_address, port) {
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
        let msg_buf = encoder::encode(packet).unwrap();
        for address in &self.send_addresses {
            self.socket.as_ref().unwrap().send_to(&msg_buf, address).unwrap();
        }
        self.send_addresses.len()
    }

    fn send_bundle(&self, bundle: &OscBundle) -> usize {
        for address in &self.send_addresses {
            for packet in &bundle.content {
                let msg_buf = encoder::encode(packet).unwrap();
                self.socket.as_ref().unwrap().send_to(&msg_buf, address).unwrap();
            }
        }
        self.send_addresses.len()
    }

    pub fn push_send_address(&mut self, ip_address: IpAddr, port: u16) -> bool {
        let address = SocketAddrV4::new(
            Ipv4Addr::from_str(&ip_address.to_string()).unwrap(),
            port
        );
        let found = self.send_addresses.iter()
            .find(|&send_address| send_address.to_string() == address.to_string());
        if found.is_none() {
            self.send_addresses.push(address);
            return true;
        }
        false
    }

    pub fn remove_send_address(&mut self, ip_address: IpAddr, port: u16) -> bool {
        let address = SocketAddrV4::new(
            Ipv4Addr::from_str(&ip_address.to_string()).unwrap(),
            port
        );
        self.send_addresses
            .retain(|&send_address| {
                println!("address: {}", address);
                println!("send_address: {}", send_address);
                send_address.to_string() != address.to_string()
            });
        true
    }

    fn convert_connection_message_to_port_number(&mut self, message: &OscMessage) -> Result<u16, OscError> {
        if message.args.is_empty() {
            return Ok(self.send_port)
        }

        match message.args[0].clone() {
            OscType::Int(i) => {
                Ok(i as u16)
            },
            _ => {
                println!("{:?}", message);
                let error_message = format!("Expected type int, but received {:?}", message.args[0]);
                Err(OscError::BadArg(error_message))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::str::FromStr;
    use crate::broadcaster::BroadCaster;
    use crate::broadcaster::core::{DEFAULT_IP_ADDRESS, DEFAULT_PORT, DEFAULT_SEND_PORT};

    #[test]
    fn test_new() {
        let broadcaster = BroadCaster::new();
        assert_eq!(0, broadcaster.send_addresses.len());
        assert_eq!(DEFAULT_IP_ADDRESS, broadcaster.listen_ip_address);
        assert_eq!(DEFAULT_PORT, broadcaster.listen_port);
        assert_eq!(DEFAULT_SEND_PORT, broadcaster.send_port);
        assert!(broadcaster.socket.is_none());
    }

    #[test]
    fn test_push_send_address() {
        let mut broadcaster = BroadCaster::new();
        assert_eq!(0, broadcaster.send_addresses.len());
        let result = broadcaster.push_send_address(
            IpAddr::from_str("127.0.0.1").unwrap(),
            33333
        );
        assert!(result);
        assert_eq!(1, broadcaster.send_addresses.len());
        assert_eq!("127.0.0.1", broadcaster.send_addresses[0].ip().to_string());
        assert_eq!(33333, broadcaster.send_addresses[0].port());
    }

    #[test]
    fn test_remove_send_address() {
        let mut broadcaster = BroadCaster::new();
        assert_eq!(0, broadcaster.send_addresses.len());
        let push_result = broadcaster.push_send_address(
            IpAddr::from_str("127.0.0.1").unwrap(),
            33333
        );
        assert!(push_result);
        assert_eq!(1, broadcaster.send_addresses.len());
        assert_eq!("127.0.0.1", broadcaster.send_addresses[0].ip().to_string());
        assert_eq!(33333, broadcaster.send_addresses[0].port());
        let remove_result = broadcaster.remove_send_address(
            IpAddr::from_str("127.0.0.1").unwrap(),
            33333
        );
        assert!(remove_result);
        assert_eq!(0, broadcaster.send_addresses.len());
    }
}
