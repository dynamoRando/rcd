use guid_create::GUID;
use rusqlite::{named_params, Connection, Result};
use crate::{crypt, sqlitedb::has_any_rows};

/*
"CREATE TABLE IF NOT EXISTS CDS_HOST_INFO
         (
             HOST_ID CHAR(36) NOT NULL,
             HOST_NAME VARCHAR(50) NOT NULL,
             TOKEN BLOB NOT NULL
         );",
 */

#[derive(Clone)]
/// Represents the information about an `rcd` instance. This data is used to identify a particular
/// `rcd` instances to others. From the perspective of *participants*, this is the *host*.
pub struct HostInfo {
    pub id: String,
    pub name: String,
    pub token: Vec<u8>,
}

impl HostInfo {
    pub fn generate(host_name: &str, conn: &Connection) {
        let id = GUID::rand();

        let token_gen = GUID::rand();
        let token = crypt::hash(&token_gen.to_string());

        let cmd = String::from(
            "
            INSERT INTO CDS_HOST_INFO
            (
                HOST_ID,
                HOST_NAME,
                TOKEN
            )
            VALUES
            (
                :id,
                :name,
                :token
            );",
        );
        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(
                named_params! {":id" : id.to_string(), ":name" : host_name, ":token" : token.0 },
            )
            .unwrap();
    }

    pub fn exists(conn: &Connection) -> bool {
        let cmd = String::from("SELECT COUNT(*) TOTALCOUNT FROM CDS_HOST_INFO");
        return has_any_rows(cmd, conn);
    }

    pub fn get(conn: &Connection) -> HostInfo {
        let cmd = String::from(
            "
        SELECT 
            HOST_ID, 
            HOST_NAME, 
            TOKEN 
        FROM 
            CDS_HOST_INFO;",
        );

        let row_to_host_info =
            |host_id: String, host_name: String, token: String| -> Result<HostInfo> {
                let host = HostInfo {
                    id: host_id,
                    name: host_name,
                    token: token.as_bytes().to_vec(),
                };

                Ok(host)
            };

        let mut results: Vec<HostInfo> = Vec::new();

        let mut statement = conn.prepare(&cmd).unwrap();
        let host_infos = statement
            .query_and_then([], |row| {
                row_to_host_info(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                )
            })
            .unwrap();

        for hi in host_infos {
            results.push(hi.unwrap());
        }

        return results.first().unwrap().clone();
    }
}
