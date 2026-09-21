package main

import (
	"context"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(2*time.Millisecond))
	defer cancel()

	ch := make(chan int)

	go func() {
		defer close(ch)
	dummy:
		for i := range 5 {
			select {
			case ch <- i:
			case <-ctx.Done():
				println("Broke from loop")
				break dummy
			}
		}
	}()

	for n := range ch {
		print(n)
	}

	println()
}
