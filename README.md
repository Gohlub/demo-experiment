# Todos for the future

- Http stubs aren't fully generated yet. Using hyperware process lib to send_request (POST) should be fairly straightforward. Extending [hyper-bindgens](https://github.com/hyperware-ai/hyper-bindgen) caller util generator with an example of how a client request works should be able to one shot it.
- Add support for `LazyLoadBlob` in a message body.
- Autogenerate function stubs from wit files in js/ts for frontend work. This is the exact same as how caller-utils is generated, except for js/ts.
- Add better logging for tests. `print_to_terminal` only works on failure, and logging could be very useful here to understand what is generally going on.
- Occasionally, even when having made changes to the code, `kit b` or `kit run-tests` doesn't trigger compilation, and one has to edit the files once more. This can cause problems with iterative development and is likely a `kit` issue.
- We currently use `record`s in the `wit` files to represent function signatures. It is entirely possible to change these to regular functions that return a future. Future support for implementing a wit component will only be available with WASI preview 3, but one can change the stub generation code to already accomodate this.
- Heavy dogfooding and performance tests.
- Test websocket client from a frontend.
