package main

import "net/http"

func main() {
    println("starting to serve you on http://localhost:8080/hello")
	http.HandleFunc("/hello", productsHandler)
	http.ListenAndServe(":8080", nil)
}

func productsHandler(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello from Go"))
}
