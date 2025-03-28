# Todos for the future

- Test message triggering one process messaging another process
- Test websocket testing
- Test state persistence with fakenode

- Add descriptive readme

---

- Add better logging for tests. `print_to_terminal` only works on failure, and logging could be very useful here to understand what is generally going on.
- Occasionally, even when having made changes to the code, `kit b` or `kit run-tests` doesn't trigger compilation, and one has to edit the files once more. This can cause problems with iterative development and is likely a `kit` issue.
- Http stubs aren't fully generated yet. Using hyperware process lib to send_request (POST) should be fairly straightforward.
- Add support for `LazyLoadBlob` in a message body.
- We currently use `record`s in the `wit` files to represent function signatures. It is entirely possible to change these to regular functions that return a future. Future support for implementing a wit component will only be available with WASI preview 3, but one can change the stub generation code to already accomodate this.
