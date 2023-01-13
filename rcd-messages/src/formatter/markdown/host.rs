use super::build_markdown_key_value_table;
use crate::client::Host;
use indexmap::IndexMap;

/// takes a Host and returns a markdown table in a key/value format
pub fn host_to_markdown_table(host: &Host) -> String {
    let mut kv: IndexMap<String, String> = IndexMap::new();

    let guid_label = "GUID: ";
    let name_label = "Host Name: ";
    let ip4_label = "IP 4: ";
    let ip6_label = "IP 6: ";
    let db_port_label = "Db Port: ";
    let token_label = "Token: ";
    let http_addr = "HTTP Addr: ";
    let http_port = "HTTP Port: ";

    let token_string = String::from_utf8_lossy(&host.token).to_owned().to_string();

    kv.insert(guid_label.to_string(), host.host_guid.clone());
    kv.insert(name_label.to_string(), host.host_name.clone());
    kv.insert(ip4_label.to_string(), host.ip4_address.clone());
    kv.insert(ip6_label.to_string(), host.ip6_address.clone());
    kv.insert(
        db_port_label.to_string(),
        host.database_port_number.to_string(),
    );
    kv.insert(token_label.to_string(), token_string);
    kv.insert(http_addr.to_string(), host.http_addr.clone());
    kv.insert(http_port.to_string(), host.http_port.to_string());

    return build_markdown_key_value_table(kv);
}
