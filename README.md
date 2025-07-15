This project shows how to use [tonic](https://github.com/hyperium/tonic) to implement both,
a gRPC client and a gRPC server defined in `.proto` files.

## gRPC Server

`/src` folder contains the server code. `build.rs` uses [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build) to compile the protos to rust code .

You can inspect the proto compilation result at `/src/proto/` folder.

`cargo run` will launch the server at `http://localhost:8080`.

Then you can use any of the clients to interact with the server or use [grpcurl](https://github.com/fullstorydev/grpcurl) or [postman](https://www.postman.com/product/grpc-client/) to interact with it.

## gRPC Clients

`clients` folder contains clients in different programming languages that communicate with the server.

See each folder for instructions on how to compile the `.proto` files and how to run the clients

### Demo

The `Dockerfile` in this project was setup to host and run this server in GPC (Google Cloud Platform) for demo purposes.