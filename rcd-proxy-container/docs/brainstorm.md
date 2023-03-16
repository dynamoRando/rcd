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

Rather than manage an RCD instance that brings online both a GRPC and HTTP endpoint, encapsulated by a container (or pod), we could extend the design of the exising RCD service to accept a _proxy_ GRPC/HTTP service on it's behalf.

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

## Other Notes

It would be preferable when a new RCD instance is created that we go ahead and generate a host-id automatically on the service itself, doing something like: 

```
// does the same thing as rcd.rs -> core.generate_host_info(host_name) 
// but instead of supplying a user specified host name, generates a GUID for the name and returns it
// this would then be used by rcd-proxy to identify the instance of RCD
service.init_host_info()
```

This would then pre-populate with a GUID for the host name. Later, if a contract is generated, we would re-generate the token and the host_name, but NEVER re-generate the id.

The host_id then becomes the method by which a rcd-proxy can identify which instance messages should be sent to.

# Alternative Design Notes

## RCD Changes
- Create new `service` method called `init_with_hash` that setup's an `RCD` instance with the specified settings
    - This method would not save the pw in plain text, instead already hashing it and saving it to the `rcd.db` with the hash.
- Create new `service` method called `start_with_config` that would override all `RcdSettings` values, including the root directory.
    - This is what would allow the concept of a "proxy" communication layer to be passed into the `RCD` instance. 
    - The idea is that the `RCD` instance technically lives only as long as the method call.

## RCD Proxy 
- Service would host an `rcd-grpc` and `rcd-http` instance. It would inspect the `auth` token for an id, look it up in it's 
    local database to see if it had one, and if it did, call the `start_with_config` method, passing in a premade `config` that 
    had it's values (it's GRPC and HTTP settings) along with the appropriate `root` dir.

- Some example methods for the "proxy" struct:
    - Create new account
        - Creates a new RCD Proxy account with a un and pw hash
        - Provisions an instance directory at the location of it's own configured `root` directory
            - Calls `init_with_hash` using the same hash as what's in the `rcd_common` "crypt" library
    - Account Table Structure 
        - username
        - pw_hash
        - instance_id 
        - dir (in theoy this should be the same name as the `instance_dir`)
            - it would be easier to just name the dir the same as the instance_id, where available
    - Test Handling Multiple Accounts
        - Test a few on-demand instances by bringing online the `start_with_config` method and ensuring everything still works
        - This would generate the config based on the inspected `id` value passed in the `AuthRequest` message.

### Proxy Steps For New Account
- Take the username and password, hash the password and save in it's own db
- If the folder doesn't already exist, cretae a new GUID for the folder name and create the directory for the account with the GUID folder name
- Create a `Settings.toml` file and place it in the directory. The username and password should be blank, because we won't use them
- Call `rcd_service.init_at_dir()` to create the needed files
- Call `rcd_service.warn_iniit_host()` to create the first time host info 

#### Proxy Steps For On Demand Work
- Call `rcd_service.start_at_existing_dir()` whenever the account logs in or a message is recieved for that instance