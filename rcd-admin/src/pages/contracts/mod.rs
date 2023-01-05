use yew::{function_component, Html, html};

use crate::pages::contracts::{active::Active, generate::Generate, pending::Pending, accepted::Accepted};

/// view the active contract on a database
pub mod active;
/// view accepted contracts for partial databases
pub mod accepted;
/// generate a new contract for a database
pub mod generate;
/// view contracts that have been sent to you to approve or reject
pub mod pending;

#[function_component]
pub fn Contracts() -> Html {
    html! {
        <div>
            <div class="container">
                <div class="box">
                < Active />
                < Accepted />
                < Generate />
                < Pending />
                </div>
            </div>
        </div>
    }
}