use std::net::{SocketAddrV4, UdpSocket};
pub mod core;
pub mod print;

pub struct BroadCaster {
    pub send_addresses: Vec<SocketAddrV4>,
    pub socket: Option<UdpSocket>,
    pub send_port: u16,
    pub listen_ip_address: String,
    pub listen_port: u16,
}
