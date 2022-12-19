use crate::RcdAdminApp;
use yew::prelude::*;
use yew::{html::Scope, Html};

pub fn view_write_behaviors(app: &RcdAdminApp, _link: &Scope<RcdAdminApp>) -> Html {
    let is_visible = !app.page.behaviors_is_visible;
    html!(
        <div hidden={is_visible}>
            <h1> {"Configure Incoming Behaviors (Update, Delete)"} </h1>
            <p>
            </p>
            </div>
      )
}
