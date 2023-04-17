use clap::Parser;
pub mod broad_caster;
use broad_caster::BroadCaster;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 32000)]
    listen_port: u16,
    #[clap(short, long, default_value_t = 12000)]
    send_port: u16,
}

fn main() {
    let args = Args::parse();
    let mut broad_caster = BroadCaster::new();
    broad_caster.set_listen_port(args.listen_port);
    broad_caster.set_send_port(args.send_port);

    println!("*** start broad cast server ***");
    println!("listening address: {}:{}", broad_caster.listen_ip_address, broad_caster.listen_port);
    println!("send port: {}", args.send_port);
    broad_caster.start();
}
