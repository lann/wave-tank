# WAVE Tank

> A laboratory setup for observing the behavior of WAVE.

```console
$ ./build-example.sh
Copied to ./example.wasm
$ cargo run -- example.wasm 'hello-world(false, "World")'
hello-world(false, some("World")) -> "Hello, World!"
```

## Completion Demo

> Making this work reliably with bash completion has proven...difficult.

```console
$ cargo run -- example.wasm --complete hello
hello
$ cargo run -- example.wasm --complete 'hello-world('
hello-world(true
hello-world(false
```
