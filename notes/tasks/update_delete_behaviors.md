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
- [ ] Overwrite With Log
- [X] Ignore

# Data Queue Notes (Unorganized)
We need a queue table for any pending changes. 

## Schema
- (Table Columns)
- Requested Date Time UTC
- Host Id
- Host Token

# Data Log Notes (Unorganized)
Use Triggers?
- https://www.sqlitetutorial.net/sqlite-trigger/
- http://souptonuts.sourceforge.net/readme_sqlite_tutorial.html
- https://stackoverflow.com/questions/422951/keeping-a-log-table-in-sqlite-database
- https://stackoverflow.com/questions/67136895/update-and-log-only-changed-rows-with-sql-in-sqlite

# Log Table Design
- Existing Columns, plus Row Id, Action (INSERT/UPDATE/DELETE), and ts (timestamp)
- [X] on CDS_HOSTS_TABLES, add flag for table USE_DATA_LOG_TABLE
- [X] Add commands (get/set) in SQLService to set flag
- Expose API for get/set of the setting
- Write tests

Note: To read the table, just use the execute read command against the participant connection.