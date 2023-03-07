pub const SQLITE_CREATE_LOGIN_TABLE: &str = "CREATE TABLE IF NOT EXISTS LOGIN
(
  username varchar(50) not null,
  hash blob not null,    
)
;";
