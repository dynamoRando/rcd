use rcd_messages::client::{AuthRequest, DatabaseSchema};
use yew::Reducible;

pub enum StateAction {
    SetAuth(String, u32, String, String),
    SetDatabases(Vec<DatabaseSchema>),
}

#[derive(Clone, Debug)]
pub struct State {
    pub auth: Auth,
    pub databases: Vec<DatabaseSchema>,
}

impl State {
    pub fn new() -> State {
        return State {
            auth: Auth::new(),
            databases: Vec::new(),
        };
    }

    pub fn database_names(&self) -> Vec<String> {
        let mut db_names: Vec<String> = Vec::new();

        for db in &self.databases {
            db_names.push(db.database_name.clone());
        }

        return db_names;
    }
}

impl Reducible for State {
    type Action = StateAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let next_self = match action {
            StateAction::SetDatabases(databases) => {
                let mut next_self = State::new();
                next_self.auth = self.auth.clone();
                next_self.databases = databases;

                next_self
            }
            StateAction::SetAuth(addr, port, un, pw) => {
                let mut next_self = State::new();
                let auth = Auth { addr, port, un, pw };
                next_self.auth = auth;
                next_self.databases = self.databases.clone();

                next_self
            }
        };

        next_self.into()
    }
}

#[derive(Clone, Debug)]
pub struct Auth {
    pub addr: String,
    pub port: u32,
    pub un: String,
    pub pw: String,
}

impl Auth {
    pub fn new() -> Auth {
        return Auth {
            addr: String::new(),
            port: 0,
            un: String::new(),
            pw: String::new(),
        };
    }

    pub fn auth_json(&self) -> String {
        let request = AuthRequest {
            user_name: self.un.to_string(),
            pw: self.pw.to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
        };

        return serde_json::to_string(&request).unwrap();
    }
}
