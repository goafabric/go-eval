package main

import "fmt"

func startPrinter() chan string {
	c := make(chan string)
	count := 0
	go func() {
		for s := range c {
			count++
			fmt.Println(count, ") ", s)
		}
	}()
	return c
}

func main() {
	c := startPrinter()

	c <- "Hello, World!"
	c <- "It works ..."
	c <- "nice eh ?"

	close(c)
}
