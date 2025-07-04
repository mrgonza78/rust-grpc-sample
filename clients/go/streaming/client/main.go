package main

import (
	"context"
	"log"
	"os"
	"time"

	proto "grpc-sample-client/protos"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	var name = "mrgonza78"
	if len(os.Args) > 1 {
		name = os.Args[1]
	}

	var addr = "localhost:50051"
	// Set up a connection to the server.
	conn, err := grpc.NewClient(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	gRPCService := proto.NewGreeterClient(conn)

	// Contact the server and print out its response.
	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	stream, err := gRPCService.HelloRequests(ctx)
	if err != nil {
		log.Fatalf("could not greet: %v", err)
	}

	for range 10 {
		err := stream.Send(&proto.HelloRequest{Name: name})
		if err != nil {
			log.Fatalf("error streaming: %v", err)
		}
	}
	response, err := stream.CloseAndRecv()
	if err != nil {
		log.Fatalf("error receiving: %v", err)
	}
	log.Printf("Response: %v", response)
}
