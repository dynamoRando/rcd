use crate::RcdAdminApp;
use yew::prelude::*;
use yew::{html::Scope, Html};

pub fn view_host_info(_app: &RcdAdminApp, _link: &Scope<RcdAdminApp>) -> Html {
    html!(
      <div>
          <h1> {"Host Info"} </h1>
          <p>
          </p>
          </div>
    )
}

pub fn view_coop_hosts(_app: &RcdAdminApp, _link: &Scope<RcdAdminApp>) -> Html {
    html!(
      <div>
          <h1> {"Cooperating Hosts"} </h1>
          <p>
          </p>
          </div>
    )
}