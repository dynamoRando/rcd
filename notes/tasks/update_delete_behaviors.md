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
- [.] in SQL Client Service, implement to modify for partial db table UPDATE/DELETE status
- in rcd SQL Client, add corresponding functions to call methods
- [.] in Data Service, when handling UPDATE/DELETE, need to query the behavior status first and respond accordingly
- [.] Write tests for each arm of the ENUM for both

# Details
## DELETE

### Deletes To Host
- [X] Send Notification
- [ ] Do Nothing

## Deletes From Host
- [X] Allow Removal
- [ ] Queue For Review
- [ ] Delete With Log
- [X] Ignore

## UPDATE

### Updates To Host
- [X] Send Data Hash Change
- [X] Do Nothing

### Updates From Host
- [ ] Allow Overwrite
- [ ] Queue For Review
- [X] Overwrite With Log
- [X] Ignore

# Data Queue Notes (Unorganized)
We need a queue table for any pending changes. We will save the pending RAW SQL update statement, to be "played" later if the participant agrees to execute it. This way, if the participant has a setting to overwrite with log, we will just tie into that logic, OR, simply just overwrite, depending on the `UpdatesFromHostBehavior` setting.

Basically, we want to replicate and/or hook into `sqlite/db_part.rs` -  `update_data_into_partial_db` command: either call `execute_update_overwrite` or `execute_update_with_log` if we accept the statement.

We will need to expose for the participant the ability to get pending updates by database and table name, and to accept or reject updates by database, table name, and queue id.

We also on the host side when we decide to shelve the UPDATE command need to send a refence and indicator to the host to let them know that their command is pending acceptance. This means changing the `proto` and also potentially having a record of pending updates on the host side (which means another table.)

## Data Queue Schema
- Id (which will be used to accept/reject the command later)
- Raw SQL Statement (TEXT)
- Requested Date Time UTC
- Host Id
- Host Token

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