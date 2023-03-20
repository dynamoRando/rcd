use yew::prelude::*;

#[function_component]
pub fn Register() -> Html {
    let ui_un = use_node_ref();
    let ui_pw = use_node_ref();

    let register_result = use_state_eq(move || String::from(""));

    html! {
        <div>
            <div class="container">
                <div class="box">
                    <div class="has-text-centered">
                        <h1 class="subtitle"> {"Register For Account"} </h1>
                        <label for="ip_address">{ "User Name" }</label>
                        <input type="text" class="input" id ="username" placeholder="username" ref={&ui_un}/>

                        <label for="port">{ "Password" }</label>
                        <input type="text" class="input"  id="pw" placeholder="pw" ref={&ui_pw} />

                        <div class="buttons">
                        <button type="button" class="button is-primary" id="register" value="Register">
                            <span class="mdi mdi-account-plus">{" Register"}</span>
                        </button>
                        </div>

                        <p>{(*register_result).clone()}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
