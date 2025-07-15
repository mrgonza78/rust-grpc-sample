This project was bootstrapped with [Create React App](https://github.com/facebook/create-react-app).

In the project directory, you can run:

```shell
$ npm start
```

Runs the app in the development mode.
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

This clients needs a special gRPC server setup that supports grpc-web.
gRPC over web is a slightly different protocol (see [grpc-web](https://github.com/grpc/grpc-web)).

In order to launch the server with grpc-web support, either checkout the `grpc-web` branch of this repository,
or make this modification to the server's `src/main.rs`:

```rust
    let chat_service = tower::ServiceBuilder::new()
        .layer(tower_http::cors::CorsLayer::permissive()) // Allowing CORS
        .layer(tonic_web::GrpcWebLayer::new()) // Using GrpcWeb layer for the react app
        .into_inner()
        .named_layer(ChatServiceServer::new(MyChatService::new()));

    Server::builder()        
        .accept_http1(true) // GrpcWeb is over http1 so we must enable it.
        .add_service(chat_service)
        .serve(addr)
        .await?;
```

Otherwise this javascript client will not be able to communicate with the server and you'll most probably see `400 bad request` responses for javascript gRPC calls

The `Dockerfile` in this project was setup to host and run this client in GPC (Google Cloud Platform) for demo purposes.