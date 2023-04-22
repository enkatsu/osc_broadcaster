use clap::Parser;
use comfy_table::Table;

pub mod broad_caster;
use broad_caster::BroadCaster;

#[derive(Parser, Debug)]
#[clap(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None)
]
struct Args {
    #[clap(short = 'i', long, default_value = "0.0.0.0")]
    listen_ip_address: String,
    #[clap(short, long, default_value_t = 32000)]
    listen_port: u16,
    #[clap(short, long, default_value_t = 12000)]
    send_port: u16,
}

fn main() {
    let args = Args::parse();
    let mut broad_caster = BroadCaster::new();
    broad_caster.set_listen_ip_address(args.listen_ip_address);
    broad_caster.set_listen_port(args.listen_port);
    broad_caster.set_send_port(args.send_port);

    let mut table = Table::new();
    table.set_header(vec!["Listening IP", "Listening PORT", "Send PORT"]);
    table.add_row(vec![
        &broad_caster.listen_ip_address,
        &broad_caster.listen_port.to_string(),
        &broad_caster.send_port.to_string(),
    ]);
    println!("{}", table);

    broad_caster.start();
}
