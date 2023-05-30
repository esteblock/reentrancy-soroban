# reentrancy-soroban

Using: PREVIEW-9
Current state of reentrancy:  not allowed

```
running 3 tests
test test::test ... ok
test test::test_reentrancy_should_panic_wanna_see_the_error ... FAILED
test test::test_reentrancy_should_panic - should panic ... ok

failures:

---- test::test_reentrancy_should_panic_wanna_see_the_error stdout ----
thread 'test::test_reentrancy_should_panic_wanna_see_the_error' panicked at 'HostError
Value: Status(HostContextError(UnknownError))

Debug events (newest first):
   0: "Debug escalating error '' to panic"
   1: "Debug contract call invocation resulted in error Status(HostContextError(UnknownError))"
   2: "Debug VM trapped with host error"
   3: "Debug escalating error '' to VM trap"
   4: "Debug contract call invocation resulted in error Status(HostContextError(UnknownError))"
   5: "Debug caught panic from contract function 'Symbol(obj#520)', propagating escalated error 'Status(HostContextError(UnknownError))'"
   6: "Debug escalating error '' to panic"
   7: "Debug contract call invocation resulted in error Status(HostContextError(UnknownError))"
   8: "Debug Contract re-entry is not allowed"
   9: "Debug no frames to derive the invoker from"
   10: ... elided ...

```

# Test by yourself

```
bash run.sh
cd event_publisher
make build
make test
cd ../vulnerable_bank
make build
make test
cd ../evil_event_publisher
make build
make test
```