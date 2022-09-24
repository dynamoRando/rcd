# Task
- implement encryption key for rcd for encrypting data in database

# Design
When a host creates a database, add the option to specify a chipher key. For a participant, upon review of a contract, give them the option to also specify a database key. A database key is the sqlchipher that will encrypt the database.

# Implementation Tasks
- On host database creation, give the option to specify a chipher key. This will be a required parameter for all requests moving forward. We should also mark in `rcd_db.db` that the database uses encryption.
- On every read, write, and cooperative write to the database at the host, we should specify the key as a parameter.
- On send to read/write/update/delete from host to participant, send the agreed upon key that will be used at the participant in the message. This will be applied to the sqlite connection to perform the operation against the database.

# Consequences of design
This design makes a major assumption that the participant
- always remembers the key provided to the host
- is comfortable with the host knowing the key and likely saving it on their side

This means that if the key is ever discovered at the host, the security is compromised. This also means that if the key is ever lost, the data is unrecoverable.

# Other Thoughts
It would be nice if there existed a design by which a participant could always be assured that they could read data in the partial database - in other words, preventing the latter situation above.

# Other Designs
Thought about using PGP for a public/private key pair. Not sure what that buys the design. 

## Unorganized Thoughts
Could we use PGP both ways? Generate a pair both for the host and the participant and use for sending messages both ways?

# What we are trying to solve for
`rcd` can be implemented anywhere, but likely will be implemented in a SaaS situation. This usually means containerization of some sort - which means that participants still don't have complete authority of the data in that they can't be sure someone hasn't copied off their data. This design spike is for trying to cover data at rest and ensuring if the container is ever compromised somehow (i.e. in order to ensure consistency that someone is using a docker volume) that while they have the data, they can't read it without the key from the participant.

