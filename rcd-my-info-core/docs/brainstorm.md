# Overview

We need software to allow us to manage multiple RCD instances on behalf of customers. An RCD instance is provisioned via a container.

Expecations:
    - rcd-my-info-core will be able to leverage both docker or k8s, depending on configuration
    - rcd-my-info will be the web page front-end to allow users to admin their rcd instance
    - rcd-admin will be the local instance for the users to admin. if possible, we don't want to 
    duplicate functionality here between rcd-admin and rcd-my-info. rcd-my-info is the SaaS that
    allows multiple users to create data accounts (implemented via rcd). rcd-admin is the software
    that allows users to actually manage their own instance of rcd.
    - do we need to leverage `nginx` to dynamically allocate domains to users?

Setup:
    - Verify that the container "rcd:latest" is available
    - Provision a container with a unique id tied to a registered user of my-info
    - Configure that container with a un and pw of the registered users' choosing
        - If possible, either shell an instance or allow a user to be able to connect to 
        that provision'd instance of rcd-admin on behalf of the user
        - If not possible, then need to break out rcd-admin into a reusable layer
        that rcd-my-info can leverage to admin an rcd instance on behalf of a user 


References: 

 - https://stackoverflow.com/questions/63128587/how-to-connect-to-a-docker-container-using-a-domain-name

# Alternative Design

Rather than manage an RCD instance that brings online both a GRPC and HTTP endpoint, encapsulated by a container (or pod), we could extend the design of the 
exising RCD service to accept a _proxy_ GRPC/HTTP service on it's behalf.

Design Highlights:
    - A new RCD proxy service that takes all the same messages that an RCD service would, along with a `instance_id` which is 
    the same RCD self id that is generated when cooperative functions are enabled.
    - We would change RCD to by default on startup always check to see if an instance id has been generated and if it has not, 
    generate one immediately. 
    - The RCD proxy service logically seperates out RCD instances by folder and `instance_id`. In other words, to know which databases
    to use, it needs to know which folder to use that contains the databases for that instance.
        - This means that `my-info` will keep track of instances via `instance_id` and directory, likely stored within the app itself.
        - It will host a proxy service to acknowledge incoming messages and route them to the appropriate instance.
    - RCD service needs to be extended to handle messages without a _transport_ layer: this is already mostly achieved via the `rcd` and 
    `rcd_data` objects which are internally known as `core` and `core_data`. This means still passing in the IP information of the proxy
    to RCD rather than getting it from the config file.
    - We need to check to make sure that the RcdData service can handle having a "proxy" reference. It may mean changing the outgoing messages
    to include an optional `instance_id` field to signal which instance the message is intended for.
    - Likewise, we need to make sure that the `Remote` object on the data service can handle using a proxy service on it's behalf. I don't believe
    it needs to (since all it does is instantiate a client) but it's worth double checking.

    