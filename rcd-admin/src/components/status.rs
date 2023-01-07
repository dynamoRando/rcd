use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct StatusProps {
    pub is_logged_in: UseStateHandle<bool>
}

#[function_component]
pub fn Status(props: &StatusProps) -> Html {
    let ui_status_message = use_node_ref();
    let is_logged_in_state = props.is_logged_in.clone();

    html!(
        <div>
            <nav class="navbar is-light">
                <div class="navbar-brand">
                    <h3 class="navbar-item is-size-4">{"Status"}</h3>
                    <div class="navbar-item">
                        <div class="buttons">
                        {
                            if *is_logged_in_state {
                                html! {
                                    <button class="button is-info">
                                        <span class="mdi mdi-account-check">{" Logged In"}</span>
                                    </button>
                                }
                            }
                            else {
                                html! {
                                    <button class="button is-light">
                                    <span class="mdi mdi-account-cancel">{" Not Logged In"}</span>
                                    </button>
                                    }
                            }
                        }
                        </div>
                    </div>
                    <div class="navbar-item">
                        <div class="field">
                        <input type="text" class="input" size=95
                        id ="status_message" placeholder="RCD Status Messages Will Appear Here"
                        ref={&ui_status_message} readonly=true />
                        </div>
                    </div>

                </div>
            </nav>
        </div>
    )
}
