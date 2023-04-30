extern crate csv;

use std::fs::File;
use std::path::PathBuf;
use broadcaster::BroadCaster;
use clap::Parser;
use csv::Reader;

pub mod broadcaster;

#[derive(Parser, Debug)]
#[clap(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None)
]
struct Args {
    /// Specify the listen IP address of the broadcast server
    #[clap(short = 'i', long, default_value = "0.0.0.0")]
    listen_ip_address: String,
    /// Specify the listen port of the broadcast server
    #[clap(short, long, default_value_t = 32000)]
    listen_port: u16,
    /// Specify the send port of the broadcast server
    #[clap(short, long, default_value_t = 12000)]
    send_port: u16,
    /// Reads the initial state of connected clients from a file
    #[clap(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn init_from_csv(broad_caster: &mut BroadCaster, mut reader: Reader<File>) {
    for record in reader.records() {
        let row = record.unwrap();
        let address = row.get(0).unwrap();
        // let port = row.get(1).unwrap();
        broad_caster.push_send_address(address.to_string());
    }
}

fn main() {
    let args = Args::parse();
    let mut broad_caster = BroadCaster::new();
    broad_caster.listen_ip_address = args.listen_ip_address;
    broad_caster.listen_port = args.listen_port;
    broad_caster.send_port = args.send_port;
    match args.file {
        Some(ref path) => {
            let mut reader = csv::Reader::from_path(path).unwrap();
            init_from_csv(&mut broad_caster, reader);
        },
        None => {
        },
    }
    broad_caster.start();
}
