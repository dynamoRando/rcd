#[path = "participant_tests/reject_host.rs"]
mod reject_host;
#[path = "participant_tests/get_host_info.rs"]
mod get_host_info;

pub mod test_harness;

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
