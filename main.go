package main

import (
	"fmt"
	"io"
	"net/http"
	"sync"
	"time"
)

func main() {
	url := "http://127.0.0.1:8080"
	workers := 12
	reqsPerWorker := 40000
	totalRequests := workers * reqsPerWorker

	// We use a custom transport to ensure Idle Connections are kept open
	// otherwise Go will open/close sockets like crazy.
	transport := &http.Transport{
		MaxIdleConns:        100,
		MaxIdleConnsPerHost: 100,
	}
	client := &http.Client{Transport: transport}

	fmt.Printf("🏃 Go Client: %d workers, %d total requests\n", workers, totalRequests)
	start := time.Now()

	var wg sync.WaitGroup
	wg.Add(workers)

	for i := 0; i < workers; i++ {
		go func() {
			defer wg.Done()
			for j := 0; j < reqsPerWorker; j++ {
				resp, err := client.Get(url)
				if err == nil {
					// In Go, you MUST drain and close the body to reuse the connection
					io.Copy(io.Discard, resp.Body)
					resp.Body.Close()
				}
			}
		}()
	}

	wg.Wait()
	duration := time.Since(start)
	rps := float64(totalRequests) / duration.Seconds()

	fmt.Println("-----------------------------------")
	fmt.Printf("Go RPS: %.2f\n", rps)
}
