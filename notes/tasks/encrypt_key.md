# Task
- implement encryption key for rcd for encrypting data in database

# Design
When a host creates a database, add the option to specify a chipher key. For a participant, upon review of a contract, give them the option to also specify a database key. A databse key is the sqlchipher that will encrypt the database.

# Implementation Tasks
- On host database creation, give the option to specify a chiper key. This will be a required parameter for all requests moving forward. We should also mark in `rcd_db.db` that the database uses encryption.
- On every read, write, and cooperative write to the database at the host, we should specify the key as a parameter.

## Design Problem

What to do about writes from the host to the participant? We do not want to give the host the key - do we want to queue the data unencrypted somewhere? Do we want to temp encrypt using the host's key and then re-encrypt using the participants key? Etc.

