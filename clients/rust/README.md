Code in this folder implements all 4 kind of gRPC clients:
* Unary calls
* and all 3 streaming calls: server, client and bidirectional streaming

There is one binary for each client, you can run them like this:

```shell
$ cargo run --bin send_message
```

The gRPC rust code generated from the `.proto` file is compiled by `build.rs` using [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build)