package main

import (
	"context"
	"io"
	"log"
	"time"

	proto "grpc-sample-client/protos"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	var addr = "localhost:8080"
	// Set up a connection to the server.
	conn, err := grpc.NewClient(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	gRPCService := proto.NewChatServiceClient(conn)

	// Contact the server and print out its response.
	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	var request = proto.HistoryRequest{StartingAt: 4}
	log.Printf("Sending: %v", &request)
	stream, err := gRPCService.GetHistory(ctx, &request)
	if err != nil {
		log.Fatalf("Error starting stream: %v", err)
	}

	for {
		resp, err := stream.Recv()
		if err == io.EOF {
			break // End of stream
		}
		if err != nil {
			log.Fatalf("Error receiving stream: %v", err)
		}
		log.Printf("Response: %v", resp)
	}
}
