use crate::{enter_event::EnterEvent, view_events::ViewEvents};
use settings::Proxy;
use std::io::Write;
use std::{env, fs::File};
use yew::prelude::*;

pub mod enter_event;
pub mod event;
pub mod event_props;
pub mod logging;
pub mod settings;
pub mod storage;
pub mod view_events;
pub mod token;
pub mod repo;

const SETTINGS_TOML: &str = "Settings.toml";
const DEFAULT_SETTINGS: &str = r#"
address = "proxy.home:50040"
account = "shark"
"#;

#[function_component]
fn App() -> Html {
    let app_state = use_state_eq(move || {
        let x: Vec<Event> = Vec::new();
        x
    });

    html!(
        <div>
            < EnterEvent events={app_state.clone()} />
            < ViewEvents events={app_state.clone()}/>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
    process_settings();
}

fn process_settings() {
    let addr = std::option_env!("ADDRESS").unwrap();
    let account = std::option_env!("ACCOUNT").unwrap();

    let settings = Proxy::new(&addr, &account);
    settings.save_to_session_storage();
}

pub fn get_current_directory() -> String {
    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap().to_string();
    cwd
}

pub fn create_defaults(path: &str) {
    let mut output = File::create(path).unwrap();
    write!(output, "{DEFAULT_SETTINGS}").unwrap();
}
