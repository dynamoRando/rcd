# Overview

This is not final, this is just brainstorming.


# Data
- /data
- /data/status
- /data/version
    - POST: `is_online`
- /data/contract/save
    - POST: `save_contract`
- data/contract/accepted-by-participant
    - POST: `participant_accepts_contract`

# Client 
- /client
- /client/status
- /client/version
    - POST: `is_online`

## Databases 
- [X] /client/databases/
    - POST: `get_databases` (this really should be a GET)
- [X] /client/databases/new/
    - POST: `create_user_database`
- [X] /client/databases/contract/generate/
    - POST: `generate_contract`
- [X] /client/databases/contract/get/
    - POST: `get_active_contract`    
- [X] /client/databases/participant/get/
    - POST: `get_participants`    
- [X] /client/databases/participant/add/
    - POST: `add_participant`
- [X] /client/databases/participant/send-contract/
    - POST: `send_participant_contract`
- [ ] /client/databases/table/
    - POST: `has_table`
- [X] /client/databases/table/policy/get/
    - POST: `get_logical_storage_policy`
- [X] /client/databases/table/policy/set/
    - POST: `set_logical_storage_policy`
- [ ] /client/databases/enable-cooperative-features
    - POST: `enable_cooperative_features`

## SQL 
- [X] /client/sql/host/read/
    - POST: `execute_read_at_host`
- [X] /client/sql/host/write/
    - POST: `execute_write_at_host`
- [ ] /client/sql/host/write/cooperative/
    - POST: `execute_cooperative_write_at_host`
- [ ] /client/sql/participant/read/
    - POST: `execute_read_at_participant`
- [ ] /client/sql/participant/write/
    - POST: `execute_write_at_participant`

## Host 
- [X] /client/host/generate
    - POST: `generate_host_info`
- /client/host/status
    - POST: `change_host_status`    

## Contract
- [X] /client/contract/review
    - POST: `review_pending_contracts`
- [X] /client/contract/accept/
    - POST: `accept_pending_contract`
- [ ] /client/contract/reject/
    - POST: `reject_pending_contract`