use yew::{html, Html, function_component, use_node_ref, Properties, UseStateHandle, Callback};

#[derive(Properties, PartialEq)]
pub struct ChangeBehaviorProps {
    pub active_database: UseStateHandle<String>,
    pub active_table: UseStateHandle<String>,
}

#[function_component]
pub fn ChangeBehavior(
    ChangeBehaviorProps {
        active_database,
        active_table,
    }: &ChangeBehaviorProps,
) -> Html {

    let ui_behavior = use_node_ref();
    let database = active_database.clone();
    let table = active_table.clone();

    let onclick = {
        Callback::from(move |_| {})
    };

    html!(
        <div>
            <p><h1 class="subtitle">{"Change Behavior"}</h1></p>
                <div class ="select is-multiple">
                <select name="set_updates_from_host_behavior" id="set_updates_from_host_behavior" ref={ui_behavior} >
                    <option value="0">{"SELECT BEHAVIOR"}</option>
                    <option value="1">{"AllowOverwrite"}</option>
                    <option value="2">{"QueueForReview"}</option>
                    <option value="3">{"OverwriteWithLog"}</option>
                    <option value="4">{"Ignore"}</option>
                    <option value="5">{"QueueForReviewAndLog"}</option>
                </select>
                </div>
                <button 
                    class="button" 
                    type="button" 
                    id="update_behavior" 
                    value="Update Behavior" 
                    onclick={onclick}>
                        <span class="mdi mdi-eject-circle">{" Update Behavior"}</span>
                </button>
        </div>
    )
}