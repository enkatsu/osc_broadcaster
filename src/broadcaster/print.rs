use std::net::SocketAddrV4;
use super::BroadCaster;
use rosc::OscMessage;
use comfy_table::Table;

impl BroadCaster {
    pub fn print_send_addresses(send_addresses: &Vec<SocketAddrV4>) {
        let mut table = Table::new();
        table.set_header(vec!["IP", "PORT"]);
        for address in send_addresses {
            table.add_row(vec![
                     address.ip().to_string(),
                     address.port().to_string(),
            ]);
        }
        println!("{}", table);
    }

    pub fn print_message(message: &OscMessage) {
        let mut table = Table::new();
        let mut header = vec!["OSC Address".to_string()];
        let mut args_header: Vec<String> = (0..message.args.len())
            .map(|i| format!("Arg {:?}", i))
            .collect();
        header.append(&mut args_header);
        table.set_header(header);
        let mut row = vec![message.addr.to_string()];
        let mut args_row: Vec<String> = message.args.iter()
            .map(|msg| format!("{:?}", msg))
            .collect();
        row.append(&mut args_row);
        table.add_row(row);
        println!("{}", table);
    }
}
