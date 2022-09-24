pub mod test_harness;

#[path ="participant_tests/reject_host.rs"]
mod reject_host;
#[path ="participant_tests/delete_at_participant.rs"]
mod delete_at_participant;
#[path ="participant_tests/change_update_from_host_behavior.rs"]
mod change_update_from_host_behavior;
#[path ="participant_tests/change_delete_from_host_behavior.rs"]
mod change_delete_from_host_behavior;
/* 
# Test Module Overview

This module is intended to group tests related to expectations for a participant. 

## Test Module Background
We want participants to have full authority over their data. This means many things, for example, once a participant agrees to cooperate with a host, we
should be able to do things such as:

- later reject cooperating with a host
- change how we want to accept data changes from a host (UPDATE/DELETE)

and so on.

*/