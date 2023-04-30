use std::fs::File;
use std::io::{BufReader, Read};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::net::IpAddr;
use std::str::FromStr;
use::csv;
use crate::broadcaster::BroadCaster;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    clients: Vec<Client>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Client {
    address: String,
    port: u16,
}

fn init_from_csv(broad_caster: &mut BroadCaster, path: &PathBuf) {
    let mut reader = csv::Reader::from_path(path).unwrap();
    for record in reader.deserialize() {
        let row: Client = record.unwrap();
        broad_caster.push_send_address(
            IpAddr::from_str(&row.address).unwrap(),
            row.port
        );
    }
}

fn init_from_json(broad_caster: &mut BroadCaster, path: &PathBuf) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader::<BufReader<File>, Config>(reader).unwrap();
    for client in config.clients {
        broad_caster.push_send_address(
            IpAddr::from_str(&client.address).unwrap(),
            client.port
        );
    }
}

fn init_from_yaml(broad_caster: &mut BroadCaster, path: &PathBuf) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader).unwrap();
    for client in config.clients {
        broad_caster.push_send_address(
            IpAddr::from_str(&client.address).unwrap(),
            client.port
        );
    }
}

fn init_from_toml(broad_caster: &mut BroadCaster, path: &PathBuf) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut toml_string = String::new();
    reader.read_to_string(&mut toml_string).unwrap();
    let config: Config = toml::from_str(&toml_string).unwrap();
    for client in config.clients {
        broad_caster.push_send_address(
            IpAddr::from_str(&client.address).unwrap(),
            client.port
        );
    }
}

pub fn init_from_file(broad_caster: &mut BroadCaster, path: &PathBuf) {
    match path.extension().unwrap().to_ascii_lowercase().to_str().unwrap() {
        "csv" => init_from_csv(broad_caster, path),
        "json" => init_from_json(broad_caster, path),
        "yaml" => init_from_yaml(broad_caster, path),
        "yml" => init_from_yaml(broad_caster, path),
        "toml" => init_from_toml(broad_caster, path),
        extension => {
            panic!("{} is not supported", extension)
        }
    }
}
