use crate::rcd_ui::PageUi;
use crate::state::databases::RcdDatabases;
use crate::state::participant::RcdParticipants;
use crate::RcdAdminApp;
use yew::prelude::*;
use yew::{html::Scope, Html};

pub mod add;
pub mod send_contract;
pub mod view;

pub fn view_participants(
    page: &PageUi,
    link: &Scope<RcdAdminApp>,
    databases: &RcdDatabases,
    participants_ui: &RcdParticipants,
) -> Html {
    let is_visible = !page.participants_is_visible;

    html!(
      <div hidden={is_visible}>
          <h1> {"Participants"} </h1>
          // view participants
          { view::view(databases, participants_ui, link) }
          // add participant
          { add::view(databases, participants_ui, link) }
          // send contract
          { send_contract::view(databases, participants_ui, link) }
          </div>
    )
}

pub fn handle_add_participant(app: &mut RcdAdminApp, ctx: &Context<RcdAdminApp>) {
    add::request(app, ctx)
}

pub fn handle_add_participant_response(
    app: &mut RcdAdminApp,
    _ctx: &Context<RcdAdminApp>,
    json_response: AttrValue,
) {
    add::response(app, _ctx, json_response)
}

pub fn handle_view_participants(app: &mut RcdAdminApp, ctx: &Context<RcdAdminApp>) {
    view::request(app, ctx)
}

pub fn handle_view_participant_response(
    app: &mut RcdAdminApp,
    _ctx: &Context<RcdAdminApp>,
    json_response: AttrValue,
) {
    view::response(app, _ctx, json_response)
}

fn get_contract_status_string(status: u32) -> String {
    match status {
        1 => "NotSent".to_string(),
        2 => "Pending".to_string(),
        3 => "Accepted".to_string(),
        4 => "Rejected".to_string(),
        _ => "Unknown".to_string(),
    }
}
