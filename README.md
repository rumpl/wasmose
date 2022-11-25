# wasmose

Silly compose-like runner for wasm modules

# Testing

First build the `fs.wasm` module:

```console
$ cd fs
$ make build
$ cp  target/wasm32-wasi/release/fs.wasm ../
```

And then you can run the `example.yml` stack:

```console
$ ./target/debug/cli example.yml # by default it uses the wasmtime runtime
...
$ ./target/debug/cli --runtime wasmedge example.yml
...
$ ./target/debug/cli --runtime wasmtime example.yml
...
```
