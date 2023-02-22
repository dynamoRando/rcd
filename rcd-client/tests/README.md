# History

Originally `rcd` was written only to support gRPC, but as I started working on trying to use web-assembly based technologies, I discovered that there were at the time no real great options for making native gRPC calls (which uses `protobuf` as the payload format).

I then out of frustration refactored `rcd` to try and also support HTTP with JSON as the payload format. Out of that rush came the now _regretful_ copy/paste madness to try and get tests to work both ways.

The current testing suite is under a full blown re-write. The new format for tests generally follow:

- Classification 
    - Action
        - `grpc.rs`
        - `http.rs`
        - `test_core.rs`
        - `mod.rs`

The idea is to place all the actual test code in `test_core` in a single function, and to make that code abstract so that it calls only on the `RcdClient` itself, which will automatically make the appropriate communication call (gRPC/HTTP) based on how it was created.

The `grpc` and `http` files hopefully should be lightweight, only specifying a test name and a contract description. Most of the setup for the communication technology has been pushed into a `TestRunner` struct to setup the tests, along with any appropriate teardown. 

Because a lot of the tests also require doing the same setup (i.e. bringing online a `main` and `participant` and ensuring they accept a database contract between them) I have tried to place common, reusable test peices in the `test_common` module.

Note that certain names of functions in various `test_core` files have the same name but will do different actions depending on the nature of the test. Do not assume that just because a function has the same name as one found in `test_common` that it will actually perform the same actions.

## Performance of Tests

I have not yet found a great solution to handling trying to bring online both a `RcdService` and `RcdClient` online, much less trying to bring online multiple instances to test both a `main` instance and it's `participant`. 

For now, tests are run in different threads, and to ensure that each `client` and `service` come up in the right order, a poor combination of `thread::sleep` occurs right now to try and ensure that a `service` has booted up and is ready to recieve messages. I had investigated trying to find a way to signal back to the parent thread a message that the service had finished coming online, but there seems to be no good method at the moment.

