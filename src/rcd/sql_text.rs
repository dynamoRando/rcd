#[allow(dead_code)]
/// Anything in CDS is in the Cooperative Data Store.
pub struct CDS {}
#[allow(dead_code)]
/// Anything in COOP are tables stored in the user database and are used
/// to enable cooperative functions with participants.
pub struct COOP {}

impl CDS {
    #[allow(dead_code)]
    /// Returns create table statement for storing users of the CDS.
    pub fn text_create_user_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS CDS_USER
        (
            USERNAME VARCHAR(25) UNIQUE,
            BYTELENGTH INT NOT NULL,
            SALT BLOB NOT NULL,
            HASH BLOB NOT NULL,
            WORKFACTOR INT NOT NULL
        );",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for storing roles of the CDS.
    pub fn text_create_role_table() -> String {
        unimplemented!();
    }

    #[allow(dead_code)]
    /// Returns create table statement for xref users to roles.
    pub fn text_create_user_role_table() -> String {
        unimplemented!();
    }

    #[allow(dead_code)]
    /// Returns create table statement for storing unique identifier to participants.
    pub fn text_create_host_info_table() -> String {
        unimplemented!();
    }

    #[allow(dead_code)]
    /// Returns create table statement for hosts that this CDS is cooperating with.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_hosts_table() -> String {
        unimplemented!();
    }

    #[allow(dead_code)]
    /// Returns create table statement for holding schema information for partial databases participating with a remote host.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_contracts_table() -> String {
        unimplemented!();
    }

    #[allow(dead_code)]
    /// Returns create table statement for holding the tables in the partial database.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_contracts_tables_table() -> String {
        unimplemented!();
    }
}
