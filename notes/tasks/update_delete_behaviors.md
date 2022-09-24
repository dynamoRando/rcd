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
- in SQL Client Service, implement to modify for partial db table UPDATE/DELETE  status
- in rcd SQL Client, add corresponding functions to call methods
- in Data Service, when handling UPDATE/DELETE, need to query the behavior status first and respond accordingly
- Write tests for each arm of the ENUM for both



