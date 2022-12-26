use rcd_messages::client::{AuthRequest, DatabaseSchema};
use yew::Reducible;

pub enum InstanceAction {
    /// addr, port, un, pw
    SetAuth(String, u32, String, String),
    SetDatabases(Vec<DatabaseSchema>),
}

/// An instance of RCD
#[derive(Clone, Debug)]
pub struct Instance {
    pub auth: Auth,
    pub databases: Vec<DatabaseSchema>,
}

impl Instance {
    pub fn new() -> Instance {
        return Instance {
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

impl Reducible for Instance {
    type Action = InstanceAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let next_self = match action {
            InstanceAction::SetDatabases(databases) => {
                let mut next_self = Instance::new();
                next_self.auth = self.auth.clone();
                next_self.databases = databases;

                next_self
            }
            InstanceAction::SetAuth(addr, port, un, pw) => {
                let mut next_self = Instance::new();
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

    pub fn addr(&self) -> String {
        return format!("{}{}{}{}", "http://", self.addr, ":", self.port);
    }
}
