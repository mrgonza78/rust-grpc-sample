package main

import (
	"context"
	"fmt"
	"log"
	"os"
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

	stream, err := gRPCService.SendBulkMessages(ctx)
	if err != nil {
		log.Fatalf("Error starting stream: %v", err)
	}

	var pid = os.Getpid()
	for i := range 100 {
		var request = proto.Message{Content: fmt.Sprintf("Hi from PID %d. This is test %d", pid, i)}
		log.Printf("Streaming: %v", &request)
		err := stream.Send(&request)
		if err != nil {
			log.Fatalf("Error streaming: %v", err)
		}
		time.Sleep(time.Second) // Sleep for 1 second between sends
	}
	response, err := stream.CloseAndRecv()
	if err != nil {
		log.Fatalf("Error ending stream: %v", err)
	}
	log.Printf("Response: %v", response)
}
