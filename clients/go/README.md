

```
$ cd clients/go
$ protoc --go_out=. --go_opt=paths=source_relative --go-grpc_out=. --go-grpc_opt=paths=source_relative --proto_path=../../src --go_opt=Mservice.proto=grpc-sample.com/protos --go-grpc_opt=Mservice.proto=grpc-sample.com/protos service.proto
```

```
$ go run unary/main.go foo
2025/07/07 10:21:22 Response: message:"Hello mrgonza78!"
```

```
$ go run streaming/server/main.go foo
2025/07/07 10:21:22 Response: message:"Hello mrgonza78!"
```