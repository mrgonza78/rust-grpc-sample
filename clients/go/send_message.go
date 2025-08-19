package main

import (
	"context"
	"crypto/tls"
	"log"
	"time"

	proto "grpc-sample-client/protos"

	"fmt"
	"os"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

func main() {
	var addr = "grpc-sample-35975833932.southamerica-west1.run.app" // Demo server
	var tlsConfig = &tls.Config{InsecureSkipVerify: true}
	// Set up a connection to the server.
	conn, err := grpc.NewClient(addr, grpc.WithTransportCredentials(credentials.NewTLS(tlsConfig)))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	gRPCService := proto.NewChatServiceClient(conn)

	// Contact the server and print out its response.
	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	var request = proto.Message{Content: fmt.Sprintf("Hi from PID %d. This is a test", os.Getpid())}
	log.Printf("Sending: %v", &request)
	response, err := gRPCService.SendMessage(ctx, &request)
	if err != nil {
		log.Fatalf("Error: %v", err)
	}
	log.Printf("Response: %v", response)
}
