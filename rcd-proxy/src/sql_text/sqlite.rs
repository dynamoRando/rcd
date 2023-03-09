pub const SQLITE_CREATE_LOGIN_TABLE: &str = "CREATE TABLE IF NOT EXISTS LOGIN
(
  username varchar(50) not null,
  hash blob not null,
  folder varchar(256),
  host_id varchar(256)    
)
;";

pub const ADD_LOGIN: &str = "INSERT INTO LOGIN 
(
  username,
  hash
)
VALUES 
(
  :un,
  :hash
)
;";

pub const UPDATE_FOLDER_FOR_LOGIN: &str =
    "UPDATE LOGIN SET folder = :folder WHERE username = :username";
pub const UPDATE_HOST_ID_FOR_LOGIN: &str =
    "UPDATE LOGIN SET host_id = :host_id WHERE username = :host_id";
