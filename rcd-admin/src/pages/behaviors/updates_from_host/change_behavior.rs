use yew::{html, Html, function_component, use_node_ref};

#[function_component]
pub fn ChangeBehavior() -> Html {

    let ui_behavior = use_node_ref();

    html!(
        <div>
            <div class="box">
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
            </div>
        </div>
    )
}