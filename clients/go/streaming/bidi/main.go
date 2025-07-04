package main

import (
	"context"
	"io"
	"log"
	"os"
	"sync"
	"time"

	proto "grpc-sample-client/protos"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func sending(stream proto.Greeter_BidiHelloClient, name string, wg *sync.WaitGroup) {
	defer wg.Done()
	for {
		err := stream.Send(&proto.HelloRequest{Name: name})
		if err != nil {
			log.Fatalf("could not send: %v", err)
		}
		time.Sleep(2 * time.Second)
	}
}

func receiving(stream proto.Greeter_BidiHelloClient, wg *sync.WaitGroup) {
	defer wg.Done()
	for {
		resp, err := stream.Recv()
		if err == io.EOF {
			break // End of stream
		}
		if err != nil {
			log.Fatalf("error receiving stream: %v", err)
		}
		log.Printf("Response: %v", resp)
	}
}

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

	stream, err := gRPCService.BidiHello(ctx)
	if err != nil {
		log.Fatalf("could not greet: %v", err)
	}

	var wg sync.WaitGroup

	wg.Add(2)
	go sending(stream, name, &wg)
	go receiving(stream, &wg)

	wg.Wait()
	log.Println("Bye")
}
