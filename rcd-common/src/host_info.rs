
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
    /*
    pub fn generate(host_name: &str, dbi: &Dbi) {
        dbi.rcd_generate_host_info(host_name);
    }

    pub fn exists(dbi: &Dbi) -> bool {
        return dbi.if_rcd_host_info_exists();
    }

    pub fn get(dbi: &Dbi) -> HostInfo {
        return dbi.rcd_get_host_info();
    }
     */
}
