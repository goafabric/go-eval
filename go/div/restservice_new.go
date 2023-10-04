package main

import "net/http"
import "net"

func main() {
    println("starting to serve you on http://localhost:8080/hello")

    listener, err := net.Listen("tcp", ":8080")
    if err != nil { panic(err) }

    server := &http.Server {
        Handler: http.HandlerFunc(productsHandler),
    }

    err = server.Serve(listener)
    if err != nil { panic(err) }
}

func productsHandler(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello from Go"))
}
