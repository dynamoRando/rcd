/*
"CREATE TABLE IF NOT EXISTS CDS_HOST_INFO
         (
             HOST_ID CHAR(36) NOT NULL,
             HOST_NAME VARCHAR(50) NOT NULL,
             TOKEN BLOB NOT NULL
         );",
 */
pub struct HostInfo{
    pub id: String,
    pub name: String,
    pub token: Vec<u8>
}