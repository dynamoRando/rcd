use crate::RcdAdminApp;
use yew::prelude::*;
use yew::{html::Scope, Html};

pub fn view_participants(app: &RcdAdminApp, _link: &Scope<RcdAdminApp>) -> Html {
    let is_visible = !app.state.page_ui.participants_is_visible;
    html!(
        <div hidden={is_visible}>
            <h1> {"Participants"} </h1>
            <p>
            </p>
            </div>
      )
}
