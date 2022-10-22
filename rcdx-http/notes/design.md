# Overview

## Problems, Intentions, Approach
I didn't realize that web assembly did not natively support compliation of most of Rust's libaries, so leveraging Yew as a front end for interacting with a `rcd` instance doesn't work (originally intended to leverage the `rcdclient` to enable admin functions.)

Specifically, things such as `tokio` leverage `socket2` which doesn't plan to target web assembly. 

The revised approach then would be to enable an HTTP API on top of `rcd` that could interface with the web assembly front end (or anything else that can interact with an HTTP API.)

I then intended to implement exposing an HTTP API directly in `rcdx`, however practially I was running into collisions between so many crates (specifically collisions between using `rocket` and `antlr` for traits.) 

It is my intent then to make just testing functionality for exposing an API quick as possible. To do that I intend to implement a binary that runs alongside `rcdx` and leverages `rcdclient` to talk to the gRPC endpoint exposed by `rcdx`.

`rcdx` will remain a binary that sits on top of various databases and enables cross talk through gRPC.

`rcdx-api` will be a binary that can be instantiated if wanted to expose an HTTP API that simply talks to `rcdx`, again using the `rcdclient` which is effectively an abstraction over gRPC.

This feels like a lot of overkill, but at the moment it's the best off the cuff idea I have.

# API Design

I don't want to duplicate effort - the `rcdp.proto` file _is_ effectively the API that I want to expose to interact with an `rcd` instance. It feels redundant to try and re-implement the API in another protocol, but I deliberately want to make the web assembly front end work, and the only way I can think to do that at this point is to try and an implement an HTTP API. 

Hopefully, this pays out in the end because it means that developers will have choices for interacting with `rcdx`: gRPC or HTTP, whichever they prefer.

I will _try_ to keep the HTTP API as congruent as possible to the `rcdp.proto` definition. The params for HTTP calls should make sense (intitially I am thinking to just make them mostly the same objects as what are defined in the proto file as JSON objects.) 

## Suggested Endpoints 

This is subject to change:

- /api
    - POST: `is_online` 
- /api/databases/
    - GET: `get_databases`
    - POST: `create_user_database`
- /api/sql/host/read
    - POST: `execute_read_at_host`
- /api/sql/host/write
    - POST: `execute_write_at_host`
- /api/sql/host/write/cooperative
    - POST: `execute_cooperative_write_at_host`


And so on...


### Alternative

#### Client
- /data

#### Client 
- /client
- /client/status
    - GET: `is_online`
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