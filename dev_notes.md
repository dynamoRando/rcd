# Unorganized Developer Notes

Just a bunch of things removed from places in code.

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