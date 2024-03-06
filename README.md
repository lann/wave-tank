# WAVE Tank

> A laboratory setup for observing the behavior of
> [WAVE](https://github.com/lann/wave).

```console
$ ./build-example.sh
Copied to ./example.wasm
$ cargo run -- example.wasm 'hello-world(false)'
hello-world(false, none) -> "Hello, world!"
$ cargo run -- example.wasm 'hello-world(true, "README")'
hello-world(true, some("README")) -> "Goodbye, README!"
```
