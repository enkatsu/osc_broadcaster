use std::fs::File;
use std::io::{BufReader, Read};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::net::IpAddr;
use std::str::FromStr;
use::csv;
use crate::broadcaster::BroadCaster;

#[derive(Serialize, Deserialize, Debug)]
struct Client {
    address: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    clients: Option<Vec<Client>>,
    listen_ip_address: Option<String>,
    listen_port: Option<u16>,
    send_port: Option<u16>,
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
    if let Some(clients) = config.clients {
        init_clients(broad_caster, clients);
    }
    if let Some(listen_ip_address) = config.listen_ip_address {
        broad_caster.listen_ip_address = listen_ip_address;
    }
    if let Some(listen_port) = config.listen_port {
        broad_caster.listen_port = listen_port;
    }
    if let Some(send_port) = config.send_port {
        broad_caster.send_port = send_port;
    }
}

fn init_from_yaml(broad_caster: &mut BroadCaster, path: &PathBuf) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader).unwrap();
    if let Some(clients) = config.clients {
        init_clients(broad_caster, clients);
    }
    if let Some(listen_ip_address) = config.listen_ip_address {
        broad_caster.listen_ip_address = listen_ip_address;
    }
    if let Some(listen_port) = config.listen_port {
        broad_caster.listen_port = listen_port;
    }
    if let Some(send_port) = config.send_port {
        broad_caster.send_port = send_port;
    }
}

fn init_from_toml(broad_caster: &mut BroadCaster, path: &PathBuf) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut toml_string = String::new();
    reader.read_to_string(&mut toml_string).unwrap();
    let config: Config = toml::from_str(&toml_string).unwrap();
    if let Some(clients) = config.clients {
        init_clients(broad_caster, clients);
    }
    if let Some(listen_ip_address) = config.listen_ip_address {
        broad_caster.listen_ip_address = listen_ip_address;
    }
    if let Some(listen_port) = config.listen_port {
        broad_caster.listen_port = listen_port;
    }
    if let Some(send_port) = config.send_port {
        broad_caster.send_port = send_port;
    }
}

fn init_clients(broad_caster: &mut BroadCaster, clients: Vec<Client>) {
    for client in clients {
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;
    use crate::broadcaster::BroadCaster;
    use crate::init::init_from_file;

    #[test]
    fn test_init_from_json_file() {
        let mut broad_caster = BroadCaster::new();
        init_from_file(&mut broad_caster, &PathBuf::from_str("./docs/setting_examples/settings.json").unwrap());
        assert_eq!("127.0.0.1", broad_caster.listen_ip_address);
        assert_eq!(32001, broad_caster.listen_port);
        assert_eq!(32002, broad_caster.send_port);
        assert_eq!(3, broad_caster.send_addresses.len());
        assert_eq!(3331, broad_caster.send_addresses[0].port());
        assert_eq!(3332, broad_caster.send_addresses[1].port());
        assert_eq!(3333, broad_caster.send_addresses[2].port());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[0].ip().to_string());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[1].ip().to_string());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[2].ip().to_string());
    }

    #[test]
    fn test_init_from_yaml_file() {
        let mut broad_caster = BroadCaster::new();
        init_from_file(&mut broad_caster, &PathBuf::from_str("./docs/setting_examples/settings.yaml").unwrap());
        assert_eq!("127.0.0.1", broad_caster.listen_ip_address);
        assert_eq!(32001, broad_caster.listen_port);
        assert_eq!(32002, broad_caster.send_port);
        assert_eq!(3, broad_caster.send_addresses.len());
        assert_eq!(3331, broad_caster.send_addresses[0].port());
        assert_eq!(3332, broad_caster.send_addresses[1].port());
        assert_eq!(3333, broad_caster.send_addresses[2].port());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[0].ip().to_string());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[1].ip().to_string());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[2].ip().to_string());
    }

    #[test]
    fn test_init_from_toml_file() {
        let mut broad_caster = BroadCaster::new();
        init_from_file(&mut broad_caster, &PathBuf::from_str("./docs/setting_examples/settings.toml").unwrap());
        assert_eq!("127.0.0.1", broad_caster.listen_ip_address);
        assert_eq!(32001, broad_caster.listen_port);
        assert_eq!(32002, broad_caster.send_port);
        assert_eq!(3, broad_caster.send_addresses.len());
        assert_eq!(3331, broad_caster.send_addresses[0].port());
        assert_eq!(3332, broad_caster.send_addresses[1].port());
        assert_eq!(3333, broad_caster.send_addresses[2].port());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[0].ip().to_string());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[1].ip().to_string());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[2].ip().to_string());
    }

    #[test]
    fn test_init_from_csv_file() {
        let mut broad_caster = BroadCaster::new();
        init_from_file(&mut broad_caster, &PathBuf::from_str("./docs/setting_examples/settings.csv").unwrap());
        assert_eq!("0.0.0.0", broad_caster.listen_ip_address);
        assert_eq!(32000, broad_caster.listen_port);
        assert_eq!(12000, broad_caster.send_port);
        assert_eq!(3, broad_caster.send_addresses.len());
        assert_eq!(3331, broad_caster.send_addresses[0].port());
        assert_eq!(3332, broad_caster.send_addresses[1].port());
        assert_eq!(3333, broad_caster.send_addresses[2].port());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[0].ip().to_string());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[1].ip().to_string());
        assert_eq!("127.0.0.1", broad_caster.send_addresses[2].ip().to_string());
    }
}
