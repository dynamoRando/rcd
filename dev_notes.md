# Unorganized Developer Notes

Just a bunch of things removed from places in code...

## Unorganized Links
- https://stackoverflow.com/questions/32900809/how-to-suppress-function-is-never-used-warning-for-a-function-used-by-tests
- https://users.rust-lang.org/t/unused-import-warning/20251

## Unorganized Code Snippets

### Client Service
These snippets are from attempts to try and spawn a testing verison of the client service.

```rust
#[tokio::main]
pub async fn start_client_async(self: &Self) -> Result<(), Box<dyn std::error::Error>> {
    info!("start_client_service");

    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap();

    client_srv::start_service(
        &self.rcd_settings.client_service_addr_port,
        &cwd,
        &self.rcd_settings.backing_database_name,
    )
}

```    
```rust
pub fn start_data_service(self: &Self) {
    info!("start_data_service");
    db_srv::start_service(&self.rcd_settings.database_service_addr_port);
}
```    

#### Using Sqlite

```rust
let mut statement = conn.prepare(&cmd).unwrap();

let row_to_tuple =
    |table_name: String, policy: i64| -> Result<(String, LogicalStoragePolicy)> {
        Ok((table_name, LogicalStoragePolicy::from_i64(policy)))
    };

let statuses = statement
    .query_and_then([], |row| {
        row_to_tuple(row.get(0).unwrap(), row.get(1).unwrap())
    })
    .unwrap();

for status in statuses {
    table_policies.push(status.unwrap());
}

return table_policies;
```

## Tests
To run and get output, try:
```
RUST_LOG=debug RUST_BACKTRACE=1 cargo test -- --nocapture
```

Test specific item with debug output:

```
RUST_LOG=debug RUST_BACKTRACE=1 cargo test save_contract -- --nocapture
```

See for more information:
https://stackoverflow.com/questions/47764448/how-to-test-grpc-apis

## Multi-threading Notes
- https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th

## Defualt Variables
```rust
let default_addr_port = "http://[::1]:50051";
```

## Random Imports

```rust
        #[cfg(test)]
        use crate::cdata::sql_client_client::SqlClientClient;
        #[cfg(test)]
        use crate::cdata::{CreateUserDatabaseRequest, TestRequest};
        #[cfg(test)]
        use log::info;
        extern crate futures;
        extern crate tokio;
        #[cfg(test)]
        use crate::test_harness;
        #[cfg(test)]
        use std::sync::mpsc;
        #[cfg(test)]
        use std::{thread, time};
```

# Table Definitions
- CDS: The core of RCD - stands for Cooperative Data Service. Tables here are common to the instance of `rcd`.
- COOP: Tables in an any rcd database instance. These tables are for managing participants as well as the metadata the participants will need.

## CDS Data Dictionary
| Table Name                    | `rcd` Struct | `cdata` Struct | Purpose                                                                                |
| ----------------------------- | ------------ | -------------- | -------------------------------------------------------------------------------------- |
| `CDS_USER`                    |              |                | Used to hold users in this `rcd` instance.                                             |
| `CDS_ROLE`                    |              |                | Holds the various roles in this instance.                                              |
| `CDS_USER_ROLE`               |              |                | Maps users to roles                                                                    |
| `CDS_HOST_INFO`               |              |                | Holds our unique identifier to participants.                                           |
| `CDS_HOSTS`                   |              |                | Holds other host info that we're cooperating with. This is used for partial databases. |
| `CDS_CONTRACTS`               |              | `Contract`     | Hold schema information for partial databases. This is info from _another_ host.       |
| `CDS_CONTRACTS_TABLES`        |              |                | Holds table schema information for a partial database.                                 |
| `CDS_CONTRACTS_TABLE_SCHEMAS` |              |                | Holds column schema for information for a partial database.                            |


## COOP Data Dictionary
| Table Name                     | `rcd` Struct              | `cdata` Struct | Purpose                                                                                                               |
| ------------------------------ | ------------------------- | -------------- | --------------------------------------------------------------------------------------------------------------------- |
| `COOP_DATA_HOST`               |                           |                | Holds the database id. Activated when cooperative features are turned on.                                             |
| `COOP_DATA_TABLES`             |                           |                | Holds the table ids. Activated when we start setting LSPs on tables. This aligns with `COOP_REMOTES`.                 |
| `COOP_DATA_HOST_TABLE_COLUMNS` |                           |                | Holds the column ids. This needs to align with the actual schema of the table.                                        |
| `COOP_REMOTES`                 |                           |                | Holds the Logical Storage Policy (LSP) setting for each table.                                                        |
| `COOP_DATABASE_CONTRACT`       | `CoopDatabaseContract`    |                | Holds contract information that we have generated for this database. This is the data that is sent _to participants_. |
| `COOP_PARTICIPANT`             | `CoopDatabaseParticipant` |                | Holds information about participants with this database.                                                              |



### COOP Data Specific Tables
| Table Name                 | Purpose                                                                                                                                                         |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `[TableName]_COOP_SHADOWS` | In addition, for every table at a host, with a LSP that is remote, there is also a `[TableName]_COOP_SHADOWS` table. This table tracks the remote participants. |
| `[TableName]_COOP_DATA`    | For every table at a host that is remote, there is a `[TableName]_COOP_DATA` that tracks the remote row at the participant.                                     |

# Design Notes
Currently we default everything to Sqlite. It may be useful to change the `SqlClientImpl` to have multiple implementations based on the backing database type - and then bring online the appropriate one based on the settings backing database type.

It may also be useful to seperate project out into different libs per backing database type:
- Sqlite
- MySql
- Postgres

And so on.


## In Flight Design Notes
- Create `dbi` as a database interface layer.
- Create a `dbi_config` layer for holding configuration settings
    - Have a Option<Config-X> for different database types
        - One for `sqlite` root folder, `postgres` connection string and login, and same for `mysql`