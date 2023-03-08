pub const SQLITE_CREATE_LOGIN_TABLE: &str = "CREATE TABLE IF NOT EXISTS LOGIN
(
  username varchar(50) not null,
  hash blob not null    
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
