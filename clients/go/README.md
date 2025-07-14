

```
$ cd clients/go
$ protoc --go_out=. --go_opt=paths=source_relative --go-grpc_out=. --go-grpc_opt=paths=source_relative --proto_path=../../src --go_opt=Mservice.proto=grpc-sample.com/protos --go-grpc_opt=Mservice.proto=grpc-sample.com/protos service.proto
```

```
$ go run send_message.go
2025/07/14 16:13:37 Response: messages_processed:1
```

```
$ go run send_message.go
...
2025/07/15 10:31:12 Response: messages_processed:100
```