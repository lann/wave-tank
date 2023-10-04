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

## Completion Demo

> Making this work reliably with bash completion has proven...difficult.

```console
$ cargo run -- example.wasm --complete hello
hello-world
$ cargo run -- example.wasm --complete 'hello-world('
hello-world(true
hello-world(false
```
