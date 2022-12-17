/*
- Connect to rcd

- Databases

- Execute SQL
- SQL Results

- Contracts
- Generate Contract

- View Active Contract

- Host Info

- View Participants
- Add Participant
- Send Contract To Participant

- Configure Incoming Behaviors


- Cooperating Hosts
*/

use self::{connection::RcdConnection, sql::RcdSql, databases::RcdDatabases, tables::RcdTables, participant::RcdParticipants, contract::RcdContract};

pub mod connection;
pub mod databases;
pub mod sql;
pub mod contract;
pub mod tables;
pub mod participant;


pub struct AdminUi {
    pub connection: RcdConnection,
    pub databases: RcdDatabases,
    pub tables: RcdTables,
    pub sql: RcdSql,
    pub participants: RcdParticipants,
    pub contract: RcdContract,
}