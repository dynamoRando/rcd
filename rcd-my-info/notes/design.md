# Overview

General plan:
    - add messages to `rcd-messages` to be seralized as JSON
    - use `rcd-proxy` as the backing instance, will host an http endpoint
        - need to design the API for this
    - use this as the front end to `rcd-proxy` to create and register accounts



# Brainstorm
- Register Account, Delete Account, etc: regular admin functions for a manged `rcd` instance
- When interacting with your `rcd` instance, the `rcd-proxy` will expose an HTTP endpoint will have a wrapper message `RcdRequest` that will take a seralized `rcd-messsage` (CreateUserDatabase, GetHostInfo, etc) as JSON and returns `RcdReply`, also a wrapper over the rcd response
    - this message is recieved at `rcd-proxy`'s HTTP endpoint, which will unwrap the underlying message and leverage the `rcd-client` lib to call `rcd-proxy`'s self-hosted (on localhost) GRPC proxy endpoint
    - all messages are returned back to `my-info` via the response recieved by the `rcd-client`, seralized and wrapped in the `RcdReply` message to be rendered on the webpage


Leverages:
- existing work done in `rcd-admin` 
    - where possible, either share the code OR if possible shell out _the same webpage_ to eliminate duplicate code in `my-info` and `rcd-admin`