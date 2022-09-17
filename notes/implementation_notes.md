# Database Implementations
| status                        | db type              |
| ----------------------------- | -------------------- |
| <input type=checkbox checked> | SQLite (In Progress) |
| <input type=checkbox>         | Postgres             |
|                               |                      |

# Status of Implementation

## Client Service
A service for handling requests from a client using a `rcd` instance.

### Function: is_online
| status                        | notes         |
| ----------------------------- | ------------- |
| <input type=checkbox checked> | Implemented   |
| <input type=checkbox checked> | Tests Written |

Recieves a test message that it will echo back to the sender. Used to determine if the services is active.

### Function: create_user_database
| status                        | notes         |
| ----------------------------- | ------------- |
| <input type=checkbox checked> | Implemented   |
| <input type=checkbox checked> | Tests Written |

Recieves a message to create a user database and creates it.

### Function: execute_read
| status                        | notes         |
| ----------------------------- | ------------- |
| <input type=checkbox checked> | Implemented   |
| <input type=checkbox checked> | Tests Written |

Recieves a SQL statement that returns a result set. If the tables in question have Logical Storage Policies other than `HostOnly` - nothing will be returned. This now works for both local reads and remote reads.

### Function: execute_write
| status                        | notes         |
| ----------------------------- | ------------- |
| <input type=checkbox checked> | Implemented   |
| <input type=checkbox checked> | Tests Written |
|                               |               |

Recieves a SQL statement that involves an INSERT/UPDATE/DELETE action for a local table. 

### Function: execute_cooperative_write
| status                        | notes                     |
| ----------------------------- | ------------------------- |
| <input type=checkbox checked> | Implemented (PARTIALLY)   |
| <input type=checkbox checked> | Tests Written (PARTIALLY) |
|                               |                           |

#### Detailed List 
| status                        | action |
| ----------------------------- | ------ |
| <input type=checkbox checked> | INSERT |
| <input type=checkbox>         | UPDATE |
| <input type=checkbox>         | DELETE |

Recieves a SQL statement that involves an INSERT/UPDATE/DELETE action for a specific participant. 

# Design Decisions
We are currently not checking the contract definition right now at the participant for actions being done from the host - we are only authenticating.

# Scenario Testing (Un-organized)
## `INSERT`s
We need to also handle inserts locally at the participant and if we should notify upstream the host of an inserted row.

## `UPDATE`s
Either the host or the participant can update records on either side of the database. If the row is stored at a participant, the following situations can occur:

### Update via Host
If the command is executed at the host and the row is stored at the participant:

- check the logical storage policy for the table where the row is being updated
- depending on the storage policy, send the request to the particpant
- the participant checks it's own definitions to see if anything has changed
- if there is agreement, update the row and the data hash and send the new hash back to the host

### Update via Participant
If the command is executed at the participant and the row is local to the participant:

- update the row immediately (because participants __always have full authority__ over their data)
- update the data hash for the row
- depending on the logical storage policy in the contract, we may or may not notify the host
    - if the contract states to notify the host, send the changed data hash and row id back to host
    - depending on the policy at the host, we may/may not update the data hash of the row at the host
    - or just simply ignore the update and let things be out of sync

## `DELETE`s
NOTE: This may need to be further refined.

Either the host or the participant can delete records on either side of the database. If the row is stored at a participant, `rcd` needs to:

- check the contract status to see if it has rights to delete the row at the participant
- if it does, issue a request for the data hash of the row
    - if the hashes match, delete the row
    - if the hashes do not match, defer action based on contract definition