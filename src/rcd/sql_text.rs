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
            HASH BLOB NOT NULL
        );",
        );
    }

    #[allow(dead_code)]
    pub fn text_add_user() -> String {
        return String::from("INSERT INTO CDS_USER (USERNAME, HASH) VALUES (:username, :hash);");
    }

    #[allow(dead_code)]
    pub fn text_get_user() -> String {
        return String::from("SELECT USERNAME, HASH FROM CDS_USER WHERE USERNAME = :un");
    }

    #[allow(dead_code)]
    pub fn text_get_user_role() -> String {
        return String::from("SELECT count(*) AS TOTALCOUNT FROM CDS_USER_ROLE WHERE USERNAME = :username AND ROLENAME = :rolename;");
    }

    #[allow(dead_code)]
    pub fn text_add_user_role() -> String {
        return String::from(
            "INSERT INTO CDS_USER_ROLE (USERNAME, ROLENAME) VALUES (:username, :rolename);",
        );
    }

    #[allow(dead_code)]
    pub fn text_get_role() -> String {
        return String::from(
            "SELECT count(*) AS ROLECOUNT FROM CDS_ROLE WHERE ROLENAME = :rolename",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for storing roles of the CDS.
    pub fn text_create_role_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS CDS_ROLE
                (
                    ROLENAME VARCHAR(25) UNIQUE
                );",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for xref users to roles.
    pub fn text_create_user_role_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS CDS_USER_ROLE
            (
                USERNAME VARCHAR(25) NOT NULL,
                ROLENAME VARCHAR(25) NOT NULL   
            );",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for storing unique identifier to participants.
    pub fn text_create_host_info_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS CDS_HOST_INFO
         (
             HOST_ID CHAR(36) NOT NULL,
             HOST_NAME VARCHAR(50) NOT NULL,
             TOKEN BLOB NOT NULL
         );",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for hosts that this CDS is cooperating with.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_hosts_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS CDS_HOSTS
        (
            HOST_ID CHAR(36) NOT NULL,
            HOST_NAME VARCHAR(50),
            TOKEN BLOB,
            IP4ADDRESS VARCHAR(25),
            IP6ADDRESS VARCHAR(25),
            PORT INT,
            LAST_COMMUNICATION_UTC DATETIME
        );",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for holding schema information for partial databases participating with a remote host.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_contracts_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS CDS_CONTRACTS
        (
            HOST_ID CHAR(36) NOT NULL,
            CONTRACT_ID CHAR(36) NOT NULL,
            CONTRACT_VERSION_ID CHAR(36) NOT NULL,
            DATABASE_NAME VARCHAR(50) NOT NULL,
            DATABASE_ID CHAR(36) NOT NULL,
            DESCRIPTION VARCHAR(255),
            GENERATED_DATE_UTC DATETIME,
            CONTRACT_STATUS INT
        );",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for holding the tables in the partial database.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_contracts_tables_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS CDS_CONTRACTS_TABLES
        (
            DATABASE_ID CHAR(36) NOT NULL,
            DATABASE_NAME VARCHAR(50) NOT NULL,
            TABLE_ID CHAR(36) NOT NULL,
            TABLE_NAME VARCHAR(50) NOT NULL
        );",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for holding the schema for the tables in the partial database.
    /// This is used for partial databases and their contracts.
    pub fn text_create_cds_contracts_tables_schemas_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS CDS_CONTRACTS_TABLE_SCHEMAS}
        (
            TABLE_ID CHAR(36) NOT NULL,
            COLUMN_ID CHAR(36) NOT NULL,
            COLUMN_NAME VARCHAR(50) NOT NULL,
            COLUMN_TYPE INT NOT NULL,
            COLUMN_LENGTH INT NOT NULL,
            COLUMN_ORDINAL INT NOT NULL,
            IS_NULLABLE INT
        );",
        );
    }
}

impl COOP {
    #[allow(dead_code)]
    /// Returns create table statement for storing the database id when we 1st enable cooperative features
    pub fn text_create_data_host_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS COOP_DATA_HOST
        (
           DATABASE_ID CHAR(36) NOT NULL,
           DATABASE_NAME VARCHAR(500) NOT NULL
        );
        ",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for storing the table ids generated when we start setting logical
    /// storage policies on tables. This should align with COOP_REMOTES.
    pub fn text_create_data_host_tables_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS COOP_DATA_TABLES
            (
                TABLE_ID CHAR(36) NOT NULL,
                TABLE_NAME VARCHAR(500) NOT NULL
            );
            ",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for storing the column ids generated when we start setting logical
    /// storage policies on tables. This should align with the actual schema of the table in the datbase.
    pub fn text_create_data_host_tables_columns_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS COOP_DATA_HOST_TABLE_COLUMNS
            (
                TABLE_ID CHAR(36) NOT NULL,
                COLUMN_ID CHAR(36) NOT NULL,
                COLUMN_NAME VARCHAR(500) NOT NULL
            )
            ",
        );
    }

    #[allow(dead_code)]
    /// Returns SQL statement for getting the count of tables in the cooperative data table tables (this is for contracts)
    /// for the specified table name
    /// # Params:
    /// - ":table_name"
    pub fn text_get_count_from_data_host_tables_for_table(table_name: &str) -> String {
        let mut statement = String::from(
            "SELECT count(*) tablecount FROM COOP_DATA_TABLES WHERE TABLE_NAME = :table_name",
        );
        statement = statement.replace(&String::from(":table_name"), &table_name);
        return statement;
    }

    #[allow(dead_code)]
    pub fn text_get_count_from_data_host() -> String {
        return String::from("SELECT COUNT(*) COUNT FROM COOP_DATA_HOST");
    }

    #[allow(dead_code)]
    /// adds the generated database_id and database_name to the COOP_DATA_HOST table
    /// # Params:
    /// - ":database_id"
    /// - ":database_name"
    pub fn text_add_database_id_to_host() -> String {
        return String::from(
            "INSERT INTO COOP_DATA_HOST
        (DATABASE_ID, DATABASE_NAME) VALUES (:database_id, :database_name);",
        );
    }

    #[allow(dead_code)]
    /// Returns create table statement for storing the logcial storage policy for each table
    pub fn text_create_data_remotes_table() -> String {
        return String::from(
            "CREATE TABLE IF NOT EXISTS COOP_REMOTES
        (
            TABLENAME VARCHAR(255) NOT NULL,
            LOGICAL_STORAGE_POLICY INT NOT NULL
        );
        ;",
        );
    }

    #[allow(dead_code)]
    pub fn text_get_logical_storage_policy_tables() -> String {
        return String::from(
            "
        SELECT
            TABLENAME,
            LOGICAL_STORAGE_POLICY  
        FROM
            COOP_REMOTES
            ;
        ",
        );
    }
}
