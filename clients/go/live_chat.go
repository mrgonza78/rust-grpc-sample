package main

import (
	"context"
	"fmt"
	"io"
	"log"
	"os"
	"sync"
	"time"

	proto "grpc-sample-client/protos"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func sending(stream proto.ChatService_LiveChatClient, wg *sync.WaitGroup) {
	defer wg.Done()
	var pid = os.Getpid()
	for i := range 100 {
		var request = proto.Message{Content: fmt.Sprintf("Hi from PID %d. This is test %d", pid, i)}
		log.Printf("Streaming: %v", &request)
		err := stream.Send(&request)
		if err != nil {
			log.Fatalf("Error streaming: %v", err)
		}
		time.Sleep(2 * time.Second)
	}
}

func receiving(stream proto.ChatService_LiveChatClient, wg *sync.WaitGroup) {
	defer wg.Done()
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

	stream, err := gRPCService.LiveChat(ctx)
	if err != nil {
		log.Fatalf("could not greet: %v", err)
	}

	var wg sync.WaitGroup

	wg.Add(2)
	go sending(stream, &wg)
	go receiving(stream, &wg)

	wg.Wait()
	log.Println("Bye")
}
