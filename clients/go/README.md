Code in this folder implements all 4 kind of gRPC clients:
* Unary calls
* and all 3 streaming calls: server, client and bidirectional streaming

There is one binary for each client, you can run them like this:

```shell
$ go run send_message.go
```

The gRPC go code generated from the `.proto` files was compiled with `protoc`

```shell
$ cd clients/go
$ protoc --go_out=./protos --go_opt=paths=source_relative \
         --go_opt=Mchat.proto=grpc-sample.com/protos \
         --go-grpc_out=./protos --go-grpc_opt=paths=source_relative \
         --go-grpc_opt=Mchat.proto=grpc-sample.com/protos \
         --proto_path=../../src \
        chat.proto
```

