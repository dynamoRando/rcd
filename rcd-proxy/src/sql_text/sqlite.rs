pub const SQLITE_CREATE_LOGIN_TABLE: &str = "CREATE TABLE IF NOT EXISTS LOGIN
(
  username VARCHAR(50) not null,
  token BLOB not null,
  folder VARCHAR(256),
  host_id VARCHAR(256)    
)
;";

pub const ADD_LOGIN: &str = "INSERT INTO LOGIN 
(
  username,
  token
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

pub const GET_USER: &str = "
SELECT 
    username,
    token,
    folder,
    host_id 
FROM 
    LOGIN
WHERE 
    username = :un 
;
";

pub const UPDATE_USER: &str = "
    UPDATE LOGIN
    SET 
        folder = :folder,
        token = :hash,
        host_id = :id
    WHERE
        username = :un
;
";
