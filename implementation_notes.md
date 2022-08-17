# Notes on implementation of services

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

Recieves a SQL statement that returns a result set. If the tables in question have Logical Storage Policies other than `HostOnly` - nothing will be returned. 

Note: This functionality may be expanded to include cooperative tables. In other words, the functonality in the next function may be included in this one.

### Function: execute_cooperative_read
| status                | notes         |
| --------------------- | ------------- |
| <input type=checkbox> | Implemented   |
| <input type=checkbox> | Tests Written |

Recieves a SQL statement that involes reading a table with a logical storage policy (or policies) that span one or multiple participants.

### Function: execute_write
| status                | notes                     |
| --------------------- | ------------------------- |
| <input type=checkbox> | Implemented (PARTIALLY)   |
| <input type=checkbox> | Tests Written (PARTIALLY) |

Recieves a SQL statement that involves an INSERT/UPDATE/DELETE action. 

Note: This works for local tables. However, to be fully implemented, `rcd` must read the SQL statement and determine if the table being executed on has a logical storage policy other than `HostOnly` and act accordingly.

### Function: execute_cooperative_write
This function is deprecated in favor of double loading the previous function `execute_write` to handle both local and remote tables.

### Function: execute_write
| status                | notes                     |
| --------------------- | ------------------------- |
| <input type=checkbox> | Implemented (PARTIALLY)   |
| <input type=checkbox> | Tests Written (PARTIALLY) |

Recieves a SQL statement that involves an INSERT/UPDATE/DELETE action. 

Note: This works for local tables. However, to be fully implemented, `rcd` must read the SQL statement and determine if the table being executed on has a logical storage policy other than `HostOnly` and act accordingly.
