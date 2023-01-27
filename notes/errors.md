
# Create New Database
If you create a new database using rcd-admin, and then you navigate to the "SQL" page, you will get the following errors:

thread 'rocket-worker-thread' panicked at 'called `Result::unwrap()` on an `Err` value: SqliteFailure(Error { code: Unknown, extended_code: 1 }, Some("no such table: COOP_PARTICIPANT"))', rcd-sqlite/src/sqlite/db/participant.rs:366:43

The page assumes that there are participants in the database, but if this is a newly created database and cooperative features
have not been enabled, then the table won't exist yet.

Remediation: We should check to see if the table exists, and if it doesnt, we should log a warning. In addition, we should only
return a list of participants for database that have cooperative features enabled.

