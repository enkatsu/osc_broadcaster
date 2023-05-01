use std::path::PathBuf;
use clap::Parser;

mod broadcaster;
mod init;

#[derive(Parser, Debug)]
#[clap(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = env!("CARGO_PKG_HOMEPAGE"),
)]
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
    /// Reads the initial state of connected clients from a file (JSON, YAML, TOML, CSV)
    #[clap(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let mut broad_caster = broadcaster::BroadCaster::new();
    broad_caster.listen_ip_address = args.listen_ip_address;
    broad_caster.listen_port = args.listen_port;
    broad_caster.send_port = args.send_port;
    if let Some(path) = args.file {
        if !path.exists() {
            panic!("{:?} is not found", path);
        }
        init::init_from_file(&mut broad_caster, &path);
    }
    broad_caster.start();
}
