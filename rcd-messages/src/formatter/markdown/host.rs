use crate::client::Host;

pub fn host_to_markdown_table(host: &Host) -> String {
    let mut markdown_table = String::new();
    let mut total_length_of_table = 0;

    let guid_label = "GUID: ";
    let name_label = "Host Name: ";
    let ip4_label =  "IP 4: ";
    let ip6_label =  "IP 6: ";
    let db_port_label = "Db Port: ";
    let token_label = "Token: ";

    todo!()
}