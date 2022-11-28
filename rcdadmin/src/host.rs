use crate::RcdAdminApp;
use rcd_messages::client::Host;
use yew::prelude::*;
use yew::{html::Scope, Html};

/// Shows the RCD instance's host information
pub fn view_host_info(_app: &RcdAdminApp, _link: &Scope<RcdAdminApp>) -> Html {
    html!(
      <div>
          <h1> {"Host Info"} </h1>
          <p>
          </p>
          </div>
    )
}

/// Shows RCD instances that this RCD instance is cooperating with
pub fn view_coop_hosts(_app: &RcdAdminApp, _link: &Scope<RcdAdminApp>) -> Html {
    html!(
      <div>
          <h1> {"Cooperating Hosts"} </h1>
          <p>
          </p>
          </div>
    )
}

/// takes a Host struct and returns HTML for it
pub fn view_host_html(host: &Host) -> Html {
    let host_name = &host.host_name;
    let host_ip4 = &host.ip4_address;
    let host_ip6 = &host.ip6_address;
    let host_db_port = &host.database_port_number.to_string();

    html!(
      <div>
          <h3> {"Host: "} { host_name } </h3>
          <p>
          <ul>
            <li>{"IP 4: "} { host_ip4 }</li>
            <li>{"IP 6: "} { host_ip6 }</li>
            <li>{"DB Port: "} { host_db_port }</li>
          </ul>
          </p>
          </div>
    )
}
