# rcd
"Relational Cooperative Data" or "Rust Cooperative Data."

An implementation of [dotCDS](https://github.com/dynamoRando/dotCDS), but done in Rust. I'm not a native Rust programmer, I'm learning as I go. 

CDS stands for "Cooperative Data System" or "Cooperative Data Store."

This software does not yet have a release.

# What is `rcd`
`rcd` is an intermediate API that sits on top of a database system that enables cooperation of data between the _author (or host)_ of a database and _participants_ of the database. Your application talks to `rcd`, and `rcd` talks to a local database and the corresponding _partial_ databases that are remote from the host.

`rcd` is currently being implemented on top of Sqlite, but Postgres is planned next along with MySql and Microsoft SQL Server. `rcd` is being written in Rust, and does not yet have a release.

# What is Cooperative Data
_Cooperative Data_ is a term used for the idea that usually the author of a database system often holds information that usually belongs to someone else. The common example is if a database author is building an online commerce storefront. The online store probably has records for inventory and customer information. In this example, the inventory information _belongs_ to the online retailer, but the customer information (such as Shipping Address, Billing Address, and so on) belong to the _customers_.

In this situation, `rcd` enables data to physically be split in a database system. Data that belongs to the online retailer will be saved in a database created by the online retailer, and data that belongs to the customers will be saved in databases that the customers choose to host elsewhere. To both, the database schema remains the same, and ownership and behavior of data is defined by a _database contract_.

The idea for `rcd` (and other related projects) is inspired by agricultural and/or food cooperatives.

# License
This software is licensed under the Apache 2.0 license. 

Copyright 2022 Randy Le.