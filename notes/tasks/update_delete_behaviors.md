# Task
- Implement greater authority for participants over UPDATE/DELETES
    - implement "behaviors" for participants on UPDATE/DELETE

# Design
When a host sends either an UPDATE or DELETE statement to a participant, the participant has options on how to handle the request, defined by the enums:

- DeletesFromHostBehavior
- UpdatesFromHostBehavior

# Implementation Tasks
- [X] On contract generation, default to allow update and delete
- [X] Modify the `cdata.proto` for new function calls for update/delete
- [X] in SQL Client Service, implement to modify for partial db table UPDATE/DELETE status
- in rcd SQL Client, add corresponding functions to call methods
- [X] in Data Service, when handling UPDATE/DELETE, need to query the behavior status first and respond accordingly
- [X] Write tests for each arm of the ENUM for both

# Details
## DELETE

### Deletes To Host
- [X] Send Notification
- [X] Do Nothing

## Deletes From Host
- [X] Allow Removal
- [X] Queue For Review
- [X] Delete With Log
- [X] Ignore

## UPDATE

### Updates To Host
- [X] Send Data Hash Change
- [X] Do Nothing

### Updates From Host
- [X] Allow Overwrite
- [X] Queue For Review
- [X] Overwrite With Log
- [X] Ignore

# Data Queue Notes (Unorganized)
We need a queue table for any pending changes. We will save the pending RAW SQL update statement, to be "played" later if the participant agrees to execute it. This way, if the participant has a setting to overwrite with log, we will just tie into that logic, OR, simply just overwrite, depending on the `UpdatesFromHostBehavior` setting.

Basically, we want to replicate and/or hook into `sqlite/db_part.rs` -  `update_data_into_partial_db` command: either call `execute_update_overwrite` or `execute_update_with_log` if we accept the statement.

We will need to expose for the participant the ability to get pending updates by database and table name, and to accept or reject updates by database, table name, and queue id.

We also on the host side when we decide to shelve the UPDATE command need to send a refence and indicator to the host to let them know that their command is pending acceptance. This means changing the `proto` and also potentially having a record of pending updates on the host side (which means another table.)

## Design on sending messages back to host

Maybe add a u32 UpdateStatus field? Options would be:
- Success (covers overwrite and overwrite with log)
- Pending (covers queue for review)
- Ignored (covers ignored behavior)

Note: The `is_successful` flag is truly only for errors: meaning the database was not found, or the schema has differed in some manner, etc.

## Data Queue Schema
- Id (which will be used to accept/reject the command later)
- Raw SQL Statement (TEXT)
- Requested Date Time UTC
- Host Id
- Host Token
- IP4
- IP6

# Data Log Notes (Unorganized)
Use Triggers?
- https://www.sqlitetutorial.net/sqlite-trigger/
- http://souptonuts.sourceforge.net/readme_sqlite_tutorial.html
- https://stackoverflow.com/questions/422951/keeping-a-log-table-in-sqlite-database
- https://stackoverflow.com/questions/67136895/update-and-log-only-changed-rows-with-sql-in-sqlite

# Log Table Schema
- [X] Existing Columns, plus Row Id, Action (INSERT/UPDATE/DELETE), and ts (timestamp)
- [X] on CDS_HOSTS_TABLES, add flag for table USE_DATA_LOG_TABLE
- [X] Add commands (get/set) in SQLService to set flag
- [X] Expose API for get/set of the setting
- [X] Write tests

Note: To read the table, just use the execute read command against the participant connection.