# General Work Items
- [X] Implement UPDATE from host to participant
- [X] Implement DELETE from host to participant (this is a hard delete)
- [X] Implement greater authority for participants over UPDATE/DELETES
    - [X] implement "behaviors" for participants on UPDATE/DELETE
- [X] Write logs to a rcd_log.db 
    - modified to just write to normal log file for now using log4rs
- [X] Modify reqests from host to participant so that at the participant we check if the host has been banned
- implement a cli for rcd
    - ability to login
    - ability to list databases, etc.
    - ability to start/stop either client and/or data service
- report warning if default password has not been changed
- implement encryption key for rcd for encrypting data in database

# Deployment
## Docker 
- Have seperate images for backing databases? SQLite, Postgres? And one all encompassing installation?

# Design 
- Consider natively supporting BOTH gRPC and HTTP interactions
    - break out query parsing into a seperate lib
    - break out business logic (client and data) into seperate layer that both the gRPC and HTTP layers can talk to
    - add settings to Settings.toml to allow choice of how to interact with `rcdx`



    