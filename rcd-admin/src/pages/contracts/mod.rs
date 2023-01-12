use yew::{function_component, Html, html};

use crate::pages::contracts::{active::Active, generate::Generate, pending::Pending, accepted::Accepted, send::Send};

/// view the active contract on a database
pub mod active;
/// view accepted contracts for partial databases
pub mod accepted;
/// generate a new contract for a database
pub mod generate;
/// view contracts that have been sent to you to approve or reject
pub mod pending;
/// send contract to a participant
pub mod send; 

#[function_component]
pub fn Contracts() -> Html {
    html! {
        <div>
            <div class="container">
                <div class="box">
                <h3 class="subtitle">{"Overview"}</h3>
                <p>{"A contract is a document that contains the entire schema of a database along with the logical storage policy for each table. 
                A logical storage policy simply defines where the data will be stored in a table in the database: at the host or the participant.
                Contracts are useful because they determine how RCD will store data. They also promote data portability because they show the entire
                database schema, allowing intent and use to be inferred."}</p>
                <p>{"From this page you can view the contract on a database, generate a contract for a database, or view/accept/reject 
                contracts that have been sent to you."}</p>
                < Active />
                < Accepted />
                < Generate />
                < Pending />
                < Send />
                </div>
            </div>
        </div>
    }
}