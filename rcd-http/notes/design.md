# Overview

This is not final, this is just brainstorming.


# Data
- /data

# Client 
- /client
- /client/status
- /client/version
    - POST: `is_online`
- /client/databases/
    - GET: `get_databases`
    - POST: `create_user_database`
- /client/databases/contract/
    - POST: `generate_contract`
- /client/databases/participant/
    - POST: `add_participant`
- /client/databases/participant/cooperative/
    - POST: `send_participant_contract`
- /client/databases/table/
    - POST: `has_table`
- /client/databases/table/policy/
    - GET: `get_logical_storage_policy`
    - POST: `set_logical_storage_policy`
- /client/sql/host/read/
    - POST: `execute_read_at_host`
- /client/sql/host/write/
    - POST: `execute_write_at_host`
- /client/sql/host/write/cooperative/
    - POST: `execute_cooperative_write_at_host`
- /client/sql/participant/read/
    - POST: `execute_read_at_participant`
- /client/sql/participant/write/
    - POST: `execute_write_at_participant`