use rusqlite::{Connection, Result};

/*
"CREATE TABLE IF NOT EXISTS CDS_HOST_INFO
         (
             HOST_ID CHAR(36) NOT NULL,
             HOST_NAME VARCHAR(50) NOT NULL,
             TOKEN BLOB NOT NULL
         );",
 */

#[derive(Clone)]
pub struct HostInfo {
    pub id: String,
    pub name: String,
    pub token: Vec<u8>,
}

impl HostInfo {
    pub fn get(conn: Connection) -> HostInfo {
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
