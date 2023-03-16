# Overview

General plan:
    - add messages to `rcd-messages` to be seralized as JSON
    - use `rcd-proxy` as the backing instance, will host an http endpoint
        - need to design the API for this
    - use this as the front end to `rcd-proxy` to create and register accounts